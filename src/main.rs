mod config;
mod error;
mod stats;

use crate::stats::Metrics;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn init_log() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_line_number(false)
        .with_file(false);

    let journald_layer = tracing_journald::layer()?;
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(journald_layer)
        .init();
    Ok(())
}

async fn handle_connection(
    inbound: &mut TcpStream,
    target: String,
    name: &str,
    metrics: Arc<Mutex<Metrics>>,
) -> Result<(), io::Error> {
    let name = name.to_owned();
    let mut outbound = TcpStream::connect(target).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        let bytes = io::copy(&mut ri, &mut wo).await?;
        metrics.lock().unwrap().add_inbound(&name, bytes);
        wo.shutdown().await
    };

    let server_to_client = async {
        let bytes = io::copy(&mut ro, &mut wi).await?;
        metrics.lock().unwrap().add_outbound(&name, bytes);
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_log().await?;
    info!("Starting mrpump {}", env!("CARGO_PKG_VERSION"));
    let config = match config::load().await {
        Ok(config) => {
            info!("Sockets loaded");
            config
        }
        Err(err) => {
            error!("Failed to load config! {}", err);
            std::process::exit(2);
        }
    };
    let metrics = Arc::new(Mutex::new(Metrics::new()));

    for (name, socket) in config.sockets {
        info!("  {}: {}", name, socket);
        let listen_addr = socket.listen;
        let target_addr = socket.target;
        let metrics = metrics.clone();
        metrics.lock().unwrap().add_socket(&name);
        tokio::spawn(async move {
            match TcpListener::bind(listen_addr).await {
                Ok(listener) => loop {
                    let (mut inbound, _) = listener.accept().await.unwrap();
                    let name = name.clone();
                    let metrics = metrics.clone();
                    let target_addr = target_addr.clone();
                    tokio::spawn(async move {
                        let peer = inbound.peer_addr().unwrap();
                        info!("Accepted connection from {} to {}", &peer, &name);
                        metrics.lock().unwrap().add_connection(&name);
                        if let Err(e) =
                            handle_connection(&mut inbound, target_addr, &name, metrics.clone())
                                .await
                        {
                            error!("Failed to handle connection: {}", e);
                        }
                        info!("Dropped connection from {} to {}", &peer, &name);
                        if let Some(stats) = metrics.lock().unwrap().get(&name) {
                            info!("  {}: {}", name, stats);
                        }
                    });
                },
                Err(e) => {
                    error!("Failed to bind to {}: {}", listen_addr, e);
                }
            }
        });
    }

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

use crate::error::Error;
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing::info;

#[derive(Deserialize)]
pub struct Socket {
    pub(crate) listen: SocketAddr,
    pub(crate) target: String,
}

impl Display for Socket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.listen, self.target)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub sockets: HashMap<String, Socket>,
}

pub async fn load() -> Result<Config, Error> {
    let mut candidate_paths = vec![PathBuf::from("/etc/mrpump/config.yaml")];

    if let Some(proj_dirs) = ProjectDirs::from("", "", "mrpump") {
        candidate_paths.push(proj_dirs.config_dir().join("config.yaml"));
    }

    candidate_paths.push(PathBuf::from("config.yaml"));

    for path in candidate_paths {
        if path.exists() {
            info!("Loading configuration from {:?}", path);
            let content = tokio::fs::read_to_string(path).await?;
            let config = serde_yml::from_str(&content)?;
            return Ok(config);
        }
    }

    Err(Error::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Configuration file not found in /etc/mrpump/config.yaml, ~/.config/mrpump/config.yaml, or ./config.yaml",
    )))
}

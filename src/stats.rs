use humansize::{DECIMAL, format_size};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct ConnectionStats {
    pub inbound: u64,
    pub outbound: u64,
    pub connections: u64,
}

impl Display for ConnectionStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "inbound: {}, outbound: {}, connections: {}",
            format_size(self.inbound, DECIMAL),
            format_size(self.outbound, DECIMAL),
            self.connections
        )
    }
}

pub struct Metrics {
    stats: HashMap<String, ConnectionStats>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&ConnectionStats> {
        self.stats.get(name)
    }

    pub fn add_socket(&mut self, name: &str) {
        self.stats.insert(
            name.to_owned(),
            ConnectionStats {
                inbound: 0,
                outbound: 0,
                connections: 0,
            },
        );
    }

    pub fn add_connection(&mut self, name: &str) {
        self.stats.get_mut(name).unwrap().connections += 1;
    }

    pub fn add_inbound(&mut self, name: &str, bytes: u64) {
        self.stats.get_mut(name).unwrap().inbound += bytes;
    }

    pub fn add_outbound(&mut self, name: &str, bytes: u64) {
        self.stats.get_mut(name).unwrap().outbound += bytes;
    }
}

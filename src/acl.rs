use ipnetwork::IpNetwork;
use std::net::IpAddr;
pub enum AccessControl {
    None,
    WhiteList {
        hosts: Vec<IpAddr>,
        networks: Vec<IpNetwork>,
    },
    BlackList {
        hosts: Vec<IpAddr>,
        networks: Vec<IpNetwork>,
    },
}

impl AccessControl {
    pub fn can_pass(&self, ip: IpAddr) -> bool {
        match self {
            AccessControl::None => true,
            AccessControl::WhiteList { hosts, networks } => {
                if hosts.contains(&ip) {
                    return true;
                }
                for net in networks {
                    if net.contains(ip) {
                        return true;
                    }
                }
                false
            }
            AccessControl::BlackList { hosts, networks } => {
                if hosts.contains(&ip) {
                    return false;
                }
                for net in networks {
                    if net.contains(ip) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

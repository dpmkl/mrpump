use std::net::SocketAddr;

const TEST_PORT: u16 = 63536;

pub fn get_loopback_v4_sock_addr_base() -> SocketAddr {
    get_loopback_v4_sock_addr(TEST_PORT)
}

pub fn get_loopback_v4_sock_addr(port: u16) -> SocketAddr {
    format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap()
}
extern crate mrpump;
mod common;


#[test]
fn server_already_bound() {
    let _addr = common::get_loopback_v4_sock_addr_base();
    let _alt_names = common::get_subject_alt_names();
}

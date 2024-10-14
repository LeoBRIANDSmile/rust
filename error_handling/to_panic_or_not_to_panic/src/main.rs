use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let home : IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
    let me : IpAddr = IpAddr::V4(Ipv4Addr::new(10,1,8,95));
}

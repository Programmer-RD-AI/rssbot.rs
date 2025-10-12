use std::{net::Ipv4Addr, u8};

pub fn get_ip_split(ip: &str) -> Ipv4Addr {
    let ip_split: Vec<u8> = ip
        .split(".")
        .map(|x: &str| -> u8 {
            let int_ip_part: u8 = x.parse().unwrap();
            int_ip_part
        })
        .collect();
    Ipv4Addr::new(
        ip_split.get(0).unwrap().to_owned(),
        ip_split.get(1).unwrap().to_owned(),
        ip_split.get(2).unwrap().to_owned(),
        ip_split.get(3).unwrap().to_owned(),
    )
}

use crate::config::{DEFAULT_IP, DEFAULT_PORT};
use crate::models::RSSFeeds;
use std::fs::File;
use std::io::copy;
use std::io::stdout;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::u8;

fn get_ip_split(ip: &str) -> Ipv4Addr {
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

pub fn get_addr() -> SocketAddr {
    let env_ip = std::env::var("IP_ADDRESS");
    let ip_str: &str = env_ip.as_deref().unwrap_or(DEFAULT_IP);
    let port: u16 = std::env::var("PORT")
        .unwrap()
        .parse()
        .unwrap_or(DEFAULT_PORT);
    let ip: Ipv4Addr = get_ip_split(ip_str);
    SocketAddr::from(SocketAddrV4::new(ip, port))
}

// TODO: Cache
pub fn read_rss_feed_config_from_json(path: &str) -> RSSFeeds {
    let mut file = File::open(path).unwrap();
    let mut stdout = stdout();
    let file_content = &copy(&mut file, &mut stdout).unwrap().to_string();
    let rssfeeds: RSSFeeds = serde_json::from_str(&file_content).unwrap();
    rssfeeds
}

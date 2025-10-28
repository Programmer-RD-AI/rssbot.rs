use std::net::Ipv4Addr;

use serde::Deserialize;

use crate::config::{DEFAULT_IP, DEFAULT_PORT};

#[derive(Debug, Deserialize)]
pub struct RSSFeeds {
    pub rssfeeds: Vec<String>,
}

pub struct HyperSocketConfig {
    pub ip: String,
    pub port: u16,
}

impl Default for HyperSocketConfig {
    fn default() -> Self {
        Self {
            ip: DEFAULT_IP.to_owned(),
            port: DEFAULT_PORT,
        }
    }
}

impl ProduceIpV4Addr for HyperSocketConfig {
    fn produce(&self) -> Ipv4Addr {
        let split: Vec<u8> = self
            .ip
            .split(".")
            .map(|x: &str| -> u8 {
                let int_ip_part: u8 = x.parse().unwrap();
                int_ip_part
            })
            .collect();
        let mut ip_split: Vec<u8> = vec![];
        for (idx, split_iter) in split.iter().enumerate() {
            // ip_split.push();
        }
        Ipv4Addr::new(0, 0, 0, 0)
    }
}

trait ProduceIpV4Addr {
    fn produce(&self) -> Ipv4Addr;
}

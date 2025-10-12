use hyper::rt::Executor;
use hyper::server::conn::http2;
use rssbot::utils::get_ip_split;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;

#[derive(Clone)]
pub struct TokioExecutor;

impl<F> Executor<F> for TokioExecutor
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, fut: F) -> () {
        tokio::task::spawn(fut);
    }
}

// #[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let ip_str: &str = std::env::var("IP_ADDRESS").as_deref().unwrap_or(DEFAULT_IP);
    let port: u16 = std::env::var("PORT")
        .unwrap()
        .parse()
        .unwrap_or(DEFAULT_PORT);
    let ip: Ipv4Addr = get_ip_split(ip_str);
    let addr = SocketAddr::from(SocketAddrV4::new(ip, port));
}

// #[cfg(not(feature = "server"))]
// fn main() {
//     println!("Build without `--features server` — no server started.");
// }

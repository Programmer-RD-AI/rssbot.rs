use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::Request;
use hyper::{rt::Executor, Response};
use hyper_util::rt::TokioIo;
use rssbot::utils::get_ip_split;
use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::TcpListener;
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

#[cfg(feature = "server")]
async fn test(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[cfg(feature = "server")]
#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    dotenv::dotenv().ok();
    let env_ip = std::env::var("IP_ADDRESS");
    let ip_str: &str = env_ip.as_deref().unwrap_or(DEFAULT_IP);
    let port: u16 = std::env::var("PORT")
        .unwrap()
        .parse()
        .unwrap_or(DEFAULT_PORT);
    let ip: Ipv4Addr = get_ip_split(ip_str);
    let addr = SocketAddr::from(SocketAddrV4::new(ip, port));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(_) = http2::Builder::new(TokioExecutor)
                .serve_connection(io, service_fn(test))
                .await
            {}
        });
    }
    Ok(())
}

#[cfg(not(feature = "server"))]
fn main() {
    println!("Build without `--features server` — no server started.");
}

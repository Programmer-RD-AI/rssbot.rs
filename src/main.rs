use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::Request;
use hyper::{rt::Executor, Response};
use hyper_util::rt::TokioIo;
use rssbot::utils::get_addr;
use std::convert::Infallible;
use tokio::net::TcpListener;

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
async fn test(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[cfg(feature = "server")]
#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    dotenv::dotenv().ok();
    let addr = get_addr();
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

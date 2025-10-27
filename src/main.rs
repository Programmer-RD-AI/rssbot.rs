use hyper::rt::Executor;
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use rssbot::handlers::test::test_handler;
use rssbot::utils::get_addr;
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
                .serve_connection(io, service_fn(test_handler))
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

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::Request;
use hyper::Response;
use std::convert::Infallible;

#[cfg(feature = "server")]
pub async fn test_handler(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

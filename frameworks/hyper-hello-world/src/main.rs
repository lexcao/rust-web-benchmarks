use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use std::convert::Infallible;
use tokio::net::TcpListener;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (stream, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            let _ = Http::new().serve_connection(stream, service_fn(serve)).await;
        });
    }
}

async fn serve(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let resp = Response::new(Body::from("Hello World!"));

    Ok(resp)
}

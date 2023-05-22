use std::net::SocketAddr;

use http_body_util::{Empty, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::{service_fn};
use hyper::{Request, Response, Method, StatusCode};
use tokio::net::TcpListener;
use http_body_util::{combinators::BoxBody, BodyExt};

async fn hello(req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full("123"))),
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body().boxed())),
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

//tokio提供的annotation，简化创建异步runtime，使得async main自身成为一个async runtime
#[tokio::main]
// 返回值为Result类型，Result的泛型T为()，E为Box，Box泛型要求为实现Error,Send以及Sync的trait的struct。dyn含义为trait object
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let add = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(add).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error sering connection:{:?}", err)
            }
        });
    }
}

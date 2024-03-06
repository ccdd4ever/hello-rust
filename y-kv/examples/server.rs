use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tracing::info;
use y_kv::{CommandRequest, CommandResponse};


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;

    info!("listener start at {}",addr);
    loop {
        let (stream, cliAddr) = listener.accept().await?;
        tokio::spawn(
            async move {
                let mut stream = AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
                while let Some(Ok(msg)) = stream.next().await {
                    info!("Got new command:{:?}",msg);

                    let mut resp = CommandResponse::default();
                    resp.status = 404;
                    resp.message = "Not found".to_string();

                    stream.send(resp).await.unwrap();
                }
                info!("Client: {:?} disconnected",cliAddr)
            }
        );
    }
}
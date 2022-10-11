use futures::{sink::SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use tracing::{error, info, trace};

pub async fn new_start(addr: &str) -> anyhow::Result<()> {
    let ln = TcpListener::bind(addr).await?;

    info!("server start at: tcp://{}", addr);

    loop {
        let (stream, remote_addr) = ln.accept().await?;
        trace!("receive connect from {:?}", remote_addr);
        tokio::spawn(async move {
            let codec = codec::MagicCodec {};
            let mut frame = Framed::new(stream, codec);

            while let Some(conn) = frame.next().await {
                if let Ok(msg) = conn {
                    info!("receive info: {:?}", msg);

                    let mut copy_msg = codec::MagicMessage { ..msg };
                    copy_msg.id += 1024;

                    if let Err(err) = frame.send(copy_msg).await {
                        error!("[server] send error :{:?}", err);
                    };
                }
            }
        });
    }
}

#[cfg(test)]
mod test {

    use futures::{sink::SinkExt, StreamExt};
    use tokio::net::TcpStream;
    use tokio_util::codec::Framed;

    #[tokio::test]
    async fn test_tokio_cli() {
        let stream = TcpStream::connect("127.0.0.1:20085").await.unwrap();
        let codec = codec::MagicCodec {};
        let mut frame = Framed::new(stream, codec);

        let msg = codec::MagicMessage::new(1024, 1024, 1024);

        if let Err(err) = frame.send(msg).await {
            println!("client err:{:?}", err);
        }
    }
}

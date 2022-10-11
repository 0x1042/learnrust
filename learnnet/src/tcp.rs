use std::{
    io::Read,
    net::{SocketAddr, TcpListener},
};

use socket2::{Domain, Protocol, Socket, Type};
use tracing::debug;

pub fn start(addr_str: &str) -> anyhow::Result<()> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    let addr = addr_str.parse::<SocketAddr>()?;
    socket.bind(&addr.into())?;
    socket.listen(128)?;

    let lsn: TcpListener = socket.into();

    debug!("lsn :{:?}", lsn);
    debug!("listen at tcp://{:?}", &addr);

    loop {
        let (mut input, _addr) = lsn.accept()?;
        let mut buf = String::new();
        let res = input.read_to_string(&mut buf);
        match res {
            Ok(size) => {
                debug!("read info size: {} -> {}\n", size, &buf);
            }
            Err(err) => {
                debug!("read error {}", err);
            }
        }
    }
    // Ok(())
}

#[cfg(test)]
mod test {
    use std::{io::Write, net::SocketAddr, thread};

    use socket2::{Domain, Protocol, Socket, Type};

    use super::start;

    #[test]
    fn test_client() {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "debug")
        }
        tracing_subscriber::fmt::init();
        thread::spawn(|| start("127.0.0.1:10085").unwrap());
        let mut socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
        let addr = "127.0.0.1:10085".parse::<SocketAddr>().unwrap();
        socket.set_nodelay(true).unwrap();
        socket.set_keepalive(true).unwrap();

        socket.connect(&addr.into()).unwrap();

        for _i in 0..100 {
            let _ = socket.write("hello world".as_bytes()).unwrap();
        }
    }
}

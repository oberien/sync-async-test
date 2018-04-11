#[cfg(feature = "async")]
extern crate tokio;
#[cfg(feature = "async")]
extern crate futures;
#[cfg_attr(feature = "async", macro_use)]
#[cfg(feature = "async")]
extern crate tokio_core;

#[cfg(all(feature = "sync", feature = "tcp"))]
fn main() {
    use std::net::TcpListener;
    use std::io::Read;

    let listen = TcpListener::bind("localhost:1337").unwrap();
    let (mut con, _) = listen.accept().unwrap();
    let mut buf = [0u8; 8192];
    let mut size = 0;
    while let Ok(len) = con.read(&mut buf) {
        if len == 0 {
            break;
        }
        size += len;
    }
    println!("size: {}", size);
}

#[cfg(all(feature = "async", feature = "tcp"))]
fn main() {
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::AsyncRead;
    use futures::{Future, Poll, Async, Stream};
    use std::io;

    struct Server {
        socket: TcpStream,
        buf: Vec<u8>,
        size: usize,
    }

    impl Future for Server {
        type Item = usize;
        type Error = io::Error;

        fn poll(&mut self) -> Poll<usize, io::Error> {
            println!("poll");
            let res = try_nb!(self.socket.read(&mut self.buf);
            self.buf.clear();
            match res {
                Ok(Async::NotReady) => {
                    println!("not ready");
                    Ok(Async::NotReady)
                },
                Ok(Async::Ready(len)) => {
                    self.size += len;
                    println!("ready: {}", len);
                    Ok(Async::NotReady)
                }
                Err(e) => {
                    println!("err: {:?}", e);
                    Err(e)
                }
            }
        }
    }

    let listener = TcpListener::bind(&"127.0.0.1:1337".parse().unwrap()).unwrap();

    let server = listener.incoming()
        .and_then(|socket| Server {
            socket,
            buf: vec![0; 8192],
            size: 0
        }).map_err(|e| eprintln!("err {:?}", e))
        .for_each(|size| {
            println!("{}", size);
            futures::empty()
        });

    // Start the Tokio runtime
    tokio::run(server);
}

#[cfg(all(feature = "sync", feature = "udp"))]
fn main() {
    use std::net::UdpSocket;

    let s = UdpSocket::bind("127.0.0.1:1337").unwrap();
    let mut buf = [0u8; 8192];
    let mut size = 0;
    while let Ok((len, _)) = s.recv_from(&mut buf) {
        size += len;
    }
    println!("{}", size);
}

#[cfg(all(feature = "async", feature = "udp"))]
fn main() {
    use std::net::SocketAddr;
    use std::io;
    use tokio::net::UdpSocket;
    use tokio_core::reactor::Core;
    use futures::{Future, Poll};

    struct Server {
        socket: UdpSocket,
        buf: Vec<u8>,
        size: usize,
    }

    impl Future for &mut Server {
        type Item = ();
        type Error = io::Error;

        fn poll(&mut self) -> Poll<(), io::Error> {
            loop {
                self.size += try_nb!(self.socket.recv_from(&mut self.buf)).0;
            }
        }
    }

    let mut l = Core::new().unwrap();
    let socket = UdpSocket::bind(&"127.0.0.1:1337".parse().unwrap()).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    let mut server = Server {
        socket,
        buf: vec![0; 1024],
        size: 0,
    };
    l.run(&mut server).unwrap();
    println!("{}", server.size);
}

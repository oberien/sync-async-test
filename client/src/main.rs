#[cfg(feature = "sync")]
fn main() {
    use std::net::TcpStream;
    use std::io::Write;

    let mut s = TcpStream::connect("localhost:1337").unwrap();
    let msg = "a".repeat(150) + "\n";

    for _ in 0..5_000_000 {
        s.write_all(msg.as_bytes()).unwrap();
    }
}
#[cfg(feature = "async")]
fn main() {
    use std::net::{UdpSocket, SocketAddr};
    use std::io::Write;

    let mut s = UdpSocket::bind("127.0.0.1:0").unwrap();
    let msg = "a".repeat(150) + "\n";
    let addr: SocketAddr = "127.0.0.1:1337".parse().unwrap();

    for _ in 0..5_000_000 {
        s.send_to(msg.as_bytes(), &addr).unwrap();
    }
}

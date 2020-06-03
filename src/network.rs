use std::net::{UdpSocket, SocketAddr};

#[allow(unused_must_use)]
pub fn send_buf(server: SocketAddr, buff: &Vec<u8>) {
    let s = UdpSocket::bind("0.0.0.0:22343").expect("Failed to bind to addr");
    s.send_to(buff.as_slice(), server);
}
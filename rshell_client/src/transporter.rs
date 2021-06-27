use std::net::TcpStream;
use crate::cmd::Cmd;

pub struct Transporter<'a> {
    server: &'a str,
    port: u32,
    tcp_client: TcpStream
}

impl<'a> Transporter<'a> {
    pub fn new(server: &'a str, port: u32) -> Transporter<'a> {
        Transporter {
            server: server,
            port: port,
            tcp_client: TcpStream::connect(format!("{}:{}", server, port)).unwrap(),
        }
    }

    pub fn send(&self, cmd: Cmd) {

    }
}
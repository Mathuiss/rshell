use crate::transporter::Transporter;
use crate::cmd::Cmd;

pub struct RshpClient<'a> {
    id: u32,
    transporter: Transporter<'a>,
}

impl<'a> RshpClient<'a> {
    pub fn new(server: &str, port: u32) -> RshpClient {
        RshpClient {
            id: 0,
            transporter: Transporter::new(server, port),
        }
    }

    // Check if ID has been assigned
    // If not signed, sign in with id
    // req: GETNEWID
    // resp: SETID [int]id
    fn check_id(&self) {
        if self.id == 0 {
            let cmd = Cmd::new("GETNEWID", "");
            self.transporter.send(cmd);
        }
    }

    // Poll for new commands
    // GETCMD [int]id
    // CMD [string]buf

    // Lex command
    // lex(cmd_str: &str) -> Cmd
    
    // Parse command
    // run(cmd: Cmd)

    // If TIMEOUT [int]sec sleep for [int]sec seconds and poll again
    // If CMD [string]cmd, execute() [string]cmd
    // If CLOSE return to polling
}
pub struct Cmd {
    directive: String,
    data: String,
}

impl Cmd {
    pub fn new(directive: &str, data: &str) -> Cmd {
        Cmd {
            directive: String::from(directive),
            data: String::from(data),
        }
    }

    pub fn parse(cmd_str: String) -> Cmd {
        let mut oc = 0;
        let mut cc = 0;

        let mut r_directive = true;

        let mut directive = String::new();
        let mut data = String::new();

        for c in cmd_str.chars() {
            if c == '[' {
                oc += 1;
                continue;
            }

            if c == ']' {
                cc += 1;
                continue;
            }

            if c == ' ' {
                r_directive = false;
                continue;
            }

            if r_directive {
                directive.push(c);
            } else {
                data.push(c);
            }

            if r_directive == false && oc == cc {
                break;
            }
        }

        Cmd::new(directive.as_ref(), data.as_ref())
    }

    pub fn get_directive(&self) -> &String {
        &self.directive
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }
}
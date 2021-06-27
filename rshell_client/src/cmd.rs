pub struct Cmd<'a> {
    directive: &'a str,
    data: &'a str,
}

impl<'a> Cmd<'a> {
    pub fn new(directive: &'a str, data: &'a str) -> Cmd<'a> {
        Cmd {
            directive: directive,
            data: data,
        }
    }
}
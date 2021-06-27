extern crate duct_sh;

pub fn execute(cmd: String) -> String {
    duct_sh::sh_dangerous(cmd).read().unwrap()
}
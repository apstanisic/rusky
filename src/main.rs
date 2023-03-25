mod utils;
fn main() {
    let mut args = std::env::args();
    let action = args.nth(1).expect("Command not provided");
    if action == "install" {
        let dir = args.nth(2);
        utils::install(dir)
    } else if action == "uninstall" {
        utils::uninstall()
    } else {
        panic!("Unknown command")
    }
}

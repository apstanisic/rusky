use std::process;

mod utils;
fn main() {
    let mut args = std::env::args();
    let action = args.nth(1).expect("Command not provided");
    if action == "install" {
        let dir = args.nth(0);
        utils::install(dir)
    } else if action == "uninstall" {
        utils::uninstall()
    } else if action == "add" {
        // let all: Vec<String> = args.collect();
        // print!("{}", all.join("___"));
        let file = args.nth(0).expect("No dest folder");
        let cmd = args.nth(0).expect("No command");
        utils::add(file, cmd).unwrap();
    } else {
        println!("Unknown command");
        process::exit(1);
    }
}

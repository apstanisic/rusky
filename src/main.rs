use std::process;

mod utils;

const NO_COMMAND: &str = "Usage:
  husky install [dir] (default: .husky)
  husky uninstall
  husky set|add <file> [cmd]
  ";

fn main() {
    let mut args = std::env::args();
    let action = args.nth(1);

    if let None = action {
        println!("{}", NO_COMMAND);
        process::exit(0);
    }

    let action = action.unwrap();

    if action == "install" {
        let dir = args.nth(0);
        utils::install(dir.as_deref())
    } else if action == "uninstall" {
        utils::uninstall()
    } else if action == "add" {
        let file = args.nth(0).expect("No dest folder");
        let cmd = args.nth(0).expect("No command");
        utils::add(&file, &cmd).unwrap();
    } else {
        println!("Unknown command");
        process::exit(1);
    }
}

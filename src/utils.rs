mod shell_script;
use std::{
    env,
    fs::{self, create_dir_all, File, OpenOptions},
    io::{self, Write},
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
    process::{self, Command, Output},
    vec,
};

fn git(command: Vec<&str>) -> io::Result<Output> {
    Command::new("git").args(command).output()
    // .expect("Failed to execute git"))
}

pub fn install(dest_dir: Option<String>) -> () {
    let dir_str = match dest_dir {
        Some(v) => v,
        None => ".husky".to_string(),
    };
    let dir = PathBuf::from(dir_str.clone());

    let husky = env::var("HUSKY").unwrap_or("".to_string());
    if husky == "0" {
        println!("HUSKY env variable is set to 0, skipping install");
        return;
    }

    if !git(vec!["rev-parse"]).unwrap().status.success() {
        println!("`git command not found, skipping install`");
    }

    let help_url = "https://typicode.github.io/husky/#/?id=custom-directory";

    if !dir.starts_with(env::current_dir().unwrap()) {
        println!(".. not allowed (see {})", help_url);
        process::exit(1);
    }

    let git_folder = format!("{}/.git", dir_str);
    if !Path::new(&git_folder).exists() {
        println!(".git can't be found (see {})", help_url);
        process::exit(1);
    }

    let err_text = "Problem installing hooks.";

    create_dir_all(dir).expect(err_text);

    let mut git_ignore_file = File::create(format!("{}/_/.gitignore", dir_str)).expect(err_text);
    git_ignore_file.write_all(b"*").expect(err_text);

    let mut husky_sh_file = File::create(format!("{}/_/husky.sh", dir_str)).expect(err_text);
    husky_sh_file
        .write_all(shell_script::HUSKY_SHELL_SCRIPT.trim().as_bytes())
        .expect(err_text);

    git(vec!["config", "core.hooksPath", &dir_str]).expect(err_text);

    println!("Hooks installed successfully");
}

pub fn uninstall() -> () {
    git(vec!["config", "--unset", "core.hooksPath"]).expect("Problem removing hooks");
}

pub fn set(file_path: String, cmd: String) {
    let err_message = format!(
        "can't create hook, {} directory doesn't exist (try running husky install)",
        &file_path
    );
    let file = std::path::Path::new(&file_path);
    let dir = file.parent().expect(&err_message);
    if !dir.exists() {
        println!("{}", &err_message);
        process::exit(1);
    }
    let script_to_write = format!("{}\n{}", shell_script::USER_SCRIPT.trim(), &cmd);

    File::open(file)
        .unwrap()
        .write_all(script_to_write.as_bytes())
        .unwrap();

    fs::set_permissions(file, fs::Permissions::from_mode(0o0755)).expect(&err_message);
}

pub fn add(file_path: String, cmd: String) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&file_path).exists() {
        let mut file = OpenOptions::new().append(true).open(&file_path)?;
        file.write_all(format!("{}\n", cmd).as_bytes())?;
        println!("Updated {}", &file_path);
        return Ok(());
    } else {
        set(file_path, cmd);
        Ok(())
    }
}

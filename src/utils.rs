mod shell_script;
use std::{
    env,
    fs::{self, create_dir_all, File, OpenOptions},
    io::{self, Write},
    os::unix::prelude::PermissionsExt,
    path::PathBuf,
    process::{self, Command, Output},
    vec,
};

fn git(command: Vec<&str>) -> io::Result<Output> {
    Command::new("git").args(command).output()
    // .expect("Failed to execute git"))
}

pub fn install(dest_dir: Option<String>) -> () {
    let husky_dir = get_absolute_path(
        &dest_dir.unwrap_or(".husky".to_string()), //
    );
    let root_dir = husky_dir.clone().parent().unwrap().to_path_buf();

    if env::var("HUSKY").unwrap_or("".to_string()) == "0" {
        println!("HUSKY env variable is set to 0, skipping install");
        return;
    }

    if !git(vec!["rev-parse"]).unwrap().status.success() {
        println!("`git command not found, skipping install`");
    }

    let help_url = "https://typicode.github.io/husky/#/?id=custom-directory";

    // println!(
    //     "{}, {}",
    //     absolute.par to_str().unwrap(),
    //     env::current_dir().unwrap().to_str().unwrap()
    // );
    if !root_dir.starts_with(env::current_dir().unwrap()) {
        println!(".. not allowed (see {})", help_url);
        process::exit(1);
    }

    let git_folder = root_dir.join(".git");
    if !git_folder.exists() {
        println!(".git can't be found (see {})", help_url);
        process::exit(1);
    }

    let err_text = "Problem installing hooks.";

    let husky_underscore_folder = husky_dir.clone().join("_");
    create_dir_all(husky_underscore_folder.clone()).expect(err_text);

    let git_ignore_path = husky_underscore_folder.join(".gitignore");
    let mut git_ignore_file = File::create(&git_ignore_path).unwrap();
    git_ignore_file.write_all(b"*").expect(err_text);

    let husky_sh_file_path = husky_underscore_folder.join("husky.sh");
    let mut husky_sh_file = File::create(husky_sh_file_path).expect(err_text);

    husky_sh_file
        .write_all(shell_script::HUSKY_SHELL_SCRIPT.trim().as_bytes())
        .expect(err_text);

    let binding = husky_dir.clone();
    let git_check = binding.to_str().unwrap();
    git(vec!["config", "core.hooksPath", &git_check]).expect(err_text);

    println!("Hooks installed successfully");
}

pub fn uninstall() -> () {
    git(vec!["config", "--unset", "core.hooksPath"]).expect("Problem removing hooks");
}

pub fn set(file: PathBuf, cmd: String) {
    let err_message = format!(
        "can't create hook, {} directory doesn't exist (try running husky install)",
        &file.to_str().unwrap()
    );
    let dir = file.clone().parent().expect(&err_message).to_path_buf();
    if !dir.exists() {
        println!("{}", &err_message);
        process::exit(1);
    }
    let script_to_write = format!("{}\n{}", shell_script::USER_SCRIPT.trim(), &cmd);

    println!("{}", file.to_str().unwrap());

    File::create(file.clone())
        .expect("Can't open file")
        .write_all(script_to_write.as_bytes())
        .expect("Can't write");

    fs::set_permissions(file, fs::Permissions::from_mode(0o0755)).expect(&err_message);
}

pub fn add(file_path: String, cmd: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_absolute_path(&file_path);
    if path.exists() {
        let mut file = OpenOptions::new().append(true).open(path)?;
        writeln!(file, "{}", cmd)?;
        println!("Updated {}", &file_path);
        return Ok(());
    } else {
        set(path, cmd);
        Ok(())
    }
}

fn get_absolute_path(file_path: &str) -> PathBuf {
    let dir = PathBuf::from(file_path);
    match dir.is_absolute() {
        true => dir,
        false => env::current_dir().unwrap().join(dir),
    }
}

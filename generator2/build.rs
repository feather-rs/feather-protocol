use std::env;
use std::process::Command;

fn main() {
    clone_minecraft_data();
}

fn clone_minecraft_data() {
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/PrismarineJS/minecraft-data.git")
        .arg(&format!("{}/minecraft-data", env::var("OUT_DIR").unwrap()))
        .output()
        .expect("failed to clone minecraft-data: please ensure that git is installed");
}

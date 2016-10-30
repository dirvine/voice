use std::process::Command;

pub fn speak(string: &str) {
    Command::new("espeak")
        .arg("-p 30")
        .arg("-s 165")
        .arg("-g 3")
        .arg("-ven-sc")
        .arg(string)
        .output() // use spawn() to ignore output and not wait on completion
        .expect("espeak must be installed!");
}

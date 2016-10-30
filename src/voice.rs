use std::process::Command;

/// Speaks a string
pub fn speak(string: &str) {
    let _ = Command::new("espeak")
        .arg("-p 30")
        .arg("-s 165")
        .arg("-g 3")
        .arg("-ven-sc")
        .arg(string)
        .output()
        .expect("espeak must be installed!");
}

/// Speaks a string asyncronously
pub fn speak_async(string: &str) {
    let _ = Command::new("espeak")
        .arg("-p 30")
        .arg("-s 165")
        .arg("-g 3")
        .arg("-ven-sc")
        .arg(string)
        .spawn()
        .expect("espeak must be installed!");
}


#[cfg(test)]
extern crate time;

pub mod voice;

use voice::speak;


pub fn start() {
    speak("started Ok!");
}

#[cfg(test)]
mod tests {
    use voice::speak;
    use time;

    #[test]
    fn speak1() {
        speak("Basic test passes");
    }
    #[test]
    fn time() {
        let t = time::now().to_local();

        speak(&format!("time is now {}", t.asctime()));
    }
}

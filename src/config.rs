use toml;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use error::*;

/// Configuration struct
#[derive(Debug, RustcDecodable)]
pub struct Config {
    music_path: Option<String>,
    video_path: Option<String>,
    voice: Option<VoiceConfig>,
}

/// Voice struct
#[derive(Debug, RustcDecodable)]
struct VoiceConfig {
    voice: Option<String>,
    wake_word: Option<String>,
    wake_phonetic: Option<String>,
}

/// Try and read existing config file
/// from $HOME/.config/dirvine/voice
pub fn read() -> Result<Config> {
    let home = try!(get_config_file());
    let mut f = try!(File::open(home).chain_err(|| "Could not read config file"));
    let mut s = String::new();
    let _ = try!(f.read_to_string(&mut s).chain_err(|| "could not read config file content"));
    match toml::decode_str(&mut s) {
        Some(d) => Ok(d),
        None => return Err(From::from("cannot parse config file")),
    }
}

/// Write a defaul config if not found
pub fn write_default() -> Result<()> {
    let root = match env::home_dir().map(|x| x.join(".config/dirvine/voice/")) {
        None => return Err(From::from("cannot get Home dir")),
        Some(d) => d,
    };
    try!(fs::create_dir_all(&root).chain_err(|| "Could not create directories for config"));
    let toml = r#"
                    [Config]
                    music_path = "Music"
                    video_path = "Videos"

                    [VoiceConfig]
                    voice = "en-sc"
                    wake_word = "hey made safe"
                    wake_phonetic = "HH EY. M EY D. S EY F."

                       "#;

    // Write the file.
    let r = try!(File::create(&root.join("voice.toml"))
        .and_then(|mut file| {
            file.write_all(&toml.as_bytes())
                    .and_then(|()| file.sync_all())
        })
        .chain_err(|| "could not create config file"));
    Ok(r)
}

fn get_config_file() -> Result<PathBuf> {
    match env::home_dir().map(|x| x.join(".config/dirvine/voice/voice.toml")) {
        None => return Err(From::from("cannot get config file location")),
        Some(d) => Ok(d),
    }
}

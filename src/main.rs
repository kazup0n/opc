use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::time::SystemTime;

use std::{
    fs::OpenOptions,
    process::{Command, Stdio},
};
use toml;

#[derive(Serialize, Deserialize)]
struct TokenCache {
    token: String,
    expires_in: u64,
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn restore_cache(file: &File) -> Option<TokenCache> {
    let mut buf = Ok(BufReader::new(file))?;
    let mut line = String::new();
    buf.read_line(&mut line).ok()?;
    buf.read_line(&mut line).ok()?;
    let cache: TokenCache = toml::from_str(line.as_str()).ok()?;
    return if cache.expires_in > now() {
        Some(cache)
    } else {
        None
    };
}

fn save_cache(file: &mut File, cache: &TokenCache) -> () {
    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    let s = toml::to_string(cache).unwrap();
    write!(file, "{}", s).unwrap();
}

fn renew_token() -> TokenCache {
    let output = Command::new("op")
        .arg("signin")
        .arg("--raw")
        .arg("my")
        .stdout(Stdio::piped())
        .stdin(Stdio::inherit())
        .output()
        .unwrap();

    if output.status.success() {
        let token = std::str::from_utf8(&output.stdout).unwrap().trim_end();
        TokenCache {
            token: token.to_string(),
            expires_in: now() + 30 * 60,
        }
    } else {
        panic!(
            "err: {}",
            String::from_utf8(output.stderr.to_vec()).unwrap()
        );
    }
}

fn main() {
    //~/.config/opc/cache.toml
    let mut cache_file = dirs::home_dir().unwrap();
    cache_file.push(".config/opc/cache.toml");

    //create config dir
    std::fs::create_dir_all(cache_file.parent().unwrap()).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(cache_file)
        .unwrap();
    //check if theres's a valid cache.
    let cache = restore_cache(&file);
    let token = match cache {
        Some(c) => c,
        None => {
            let token = renew_token();
            save_cache(&mut file, &token);
            token
        }
    };
    println!("{}", token.token);
}

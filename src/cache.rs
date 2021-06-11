use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use toml;

use crate::op::TokenCache;
use crate::time;

pub struct CacheFile {
    file: File,
}

impl CacheFile {
    pub fn new() -> CacheFile {
        //~/.config/opc/cache.toml
        let mut cache_file = dirs::home_dir().unwrap();
        cache_file.push(".config/opc/cache.toml");

        //create config dir
        std::fs::create_dir_all(cache_file.parent().unwrap()).unwrap();

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(cache_file)
            .unwrap();
        return CacheFile { file };
    }

    pub fn restore_cache(&self) -> Option<TokenCache> {
        let mut buf = Ok(BufReader::new(&self.file))?;
        let mut line = String::new();
        buf.read_line(&mut line).ok()?;
        buf.read_line(&mut line).ok()?;
        let cache: TokenCache = toml::from_str(line.as_str()).ok()?;
        return if cache.expires_in > time::now() {
            Some(cache)
        } else {
            None
        };
    }

    pub fn save_cache(&mut self, cache: &TokenCache) -> () {
        self.file.set_len(0).unwrap();
        self.file.seek(SeekFrom::Start(0)).unwrap();
        let s = toml::to_string(cache).unwrap();
        write!(self.file, "{}", s).unwrap();
    }
}

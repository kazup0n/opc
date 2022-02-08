use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::PathBuf;
use toml;

use crate::op::TokenCache;
use crate::time;

use super::TokenCacheStore;

pub struct CacheFile {
    file: File,
}

impl CacheFile {
    pub fn new(mut config_dir: PathBuf) -> Box<dyn TokenCacheStore> {
        config_dir.push("cache.toml");

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(config_dir)
            .unwrap();
        Box::new(CacheFile { file })
    }
}

impl TokenCacheStore for CacheFile {
    fn restore_cache(&self) -> Option<TokenCache> {
        let mut buf = BufReader::new(&self.file);
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

    fn save_cache(&mut self, cache: &TokenCache) -> () {
        self.file.set_len(0).unwrap();
        self.file.seek(SeekFrom::Start(0)).unwrap();
        let s = toml::to_string(cache).unwrap();
        write!(self.file, "{}", s).unwrap();
    }
}

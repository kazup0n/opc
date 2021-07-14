use crate::cache_store::TokenCacheStore;
use crate::op::TokenCache;
use crate::time;
use security_framework::os::macos::keychain::{CreateOptions, SecKeychain};
use std::path::PathBuf;

pub struct KeyChainTokenCache {
    keychain: SecKeychain,
}

const SERVICE_NAME: &str = "opc";
const ACCOUNT_NAME: &str = "token_cache";

impl TokenCacheStore for KeyChainTokenCache {
    fn restore_cache(&self) -> Option<TokenCache> {
        match self
            .keychain
            .find_generic_password(SERVICE_NAME, ACCOUNT_NAME)
        {
            Err(_) => None,
            Ok((pass, _)) => {
                let line = String::from_utf8(pass.to_owned()).unwrap();
                let cache: TokenCache = toml::from_str(line.as_str()).ok()?;
                return if cache.expires_in > time::now() {
                    Some(cache)
                } else {
                    None
                };
            }
        }
    }
    fn save_cache(&mut self, cache: &TokenCache) -> () {
        let s = toml::to_string(cache).unwrap();
        self.keychain
            .set_generic_password(SERVICE_NAME, ACCOUNT_NAME, s.as_bytes())
            .unwrap();
    }
}

impl KeyChainTokenCache {
    pub fn get_or_create(mut config_dir: PathBuf) -> Box<dyn TokenCacheStore> {
        config_dir.push("cache.keychain");
        let keychain = if config_dir.exists() {
            SecKeychain::open(config_dir)
        } else {
            CreateOptions::default()
                .prompt_user(true)
                .create(config_dir)
        }
        .unwrap();

        Box::new(KeyChainTokenCache { keychain })
    }
}

use crate::op::TokenCache;

pub trait TokenCacheStore {
    fn restore_cache(&self) -> Option<TokenCache>;
    fn save_cache(&mut self, cache: &TokenCache) -> ();
}

pub mod keychain;
pub mod toml;

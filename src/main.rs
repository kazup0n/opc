mod cache_store;
mod cli;
mod op;
mod time;
use cache_store::{keychain::KeyChainTokenCache, toml::CacheFile};
use cli::CacheStoreType::{File, KeyChain};

fn main() {
    let options = cli::opt_parse();

    //~/.config/opc/
    let mut config_dir = dirs::home_dir().unwrap();
    config_dir.push(".config/opc");
    //create config dir
    std::fs::create_dir_all(config_dir.clone()).unwrap();

    let mut cache_store = match options.cache_store_type {
        KeyChain => KeyChainTokenCache::get_or_create(config_dir),
        File => CacheFile::new(config_dir),
    };

    let cache: Option<op::TokenCache> = if options.refresh {
        None
    } else {
        cache_store.restore_cache()
    };
    let token = match cache {
        Some(c) => c,
        None => {
            let token = op::TokenCache::renew_token(options.account);
            cache_store.save_cache(&token);
            token
        }
    };
    println!("{}", token.token);
}

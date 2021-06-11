mod cache;
mod op;
mod time;

fn main() {
    let mut cache_file = cache::CacheFile::new();
    //check if theres's a valid cache.
    let cache = cache_file.restore_cache();
    let token = match cache {
        Some(c) => c,
        None => {
            let token = op::TokenCache::renew_token();
            cache_file.save_cache(&token);
            token
        }
    };
    println!("{}", token.token);
}

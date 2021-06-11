mod cache;
mod cli;
mod op;
mod time;

fn main() {
    let options = cli::opt_parse();
    let mut cache_file = cache::CacheFile::new();

    let cache: Option<op::TokenCache> = if options.refresh {
        None
    } else {
        cache_file.restore_cache()
    };
    let token = match cache {
        Some(c) => c,
        None => {
            let token = op::TokenCache::renew_token(options.account);
            cache_file.save_cache(&token);
            token
        }
    };
    println!("{}", token.token);
}

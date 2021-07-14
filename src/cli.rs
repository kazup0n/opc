use clap::{arg_enum, value_t, App, Arg};

pub struct Options {
    pub account: String,
    pub refresh: bool,
    pub cache_store_type: CacheStoreType,
}

arg_enum! {
    #[derive(PartialEq, Debug)]    
    pub enum CacheStoreType {
        File,
        KeyChain,
    }
}

pub fn opt_parse() -> Options {
    let matches = App::new("opc")
        .arg(
            Arg::with_name("account")
                .value_name("ACCOUNT")
                .help("Name of account (see output of `op signin list`)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("refresh")
                .long("refresh")
                .help("force refresh token cache"),
        )
        .arg(
            Arg::with_name("store")
                .long("store")
                .possible_values(&CacheStoreType::variants())
                .default_value("KeyChain")
                .case_insensitive(true),
        )
        .get_matches();
    let account = matches.value_of("account").unwrap().to_string();
    let refresh = match matches.occurrences_of("refresh") {
        1 => true,
        _ => false,
    };
    let cache_store_type = value_t!(matches, "store", CacheStoreType).unwrap();
    Options {
        account,
        refresh,
        cache_store_type,
    }
}

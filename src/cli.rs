use clap::{App, Arg};

pub struct Options {
    pub account: String,
    pub refresh: bool,
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
        .arg(Arg::with_name("refresh").long("refresh").help("force refresh token cache"))
        .get_matches();
    let account = matches.value_of("account").unwrap().to_string();
    let refresh = match matches.occurrences_of("refresh") {
        1 => true,
        _ => false,
    };
    Options { account, refresh }
}

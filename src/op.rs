use crate::time;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize)]
pub struct TokenCache {
    pub token: String,
    pub expires_in: u64,
}

impl TokenCache {
    pub fn renew_token(account: String) -> TokenCache {
        let output = Command::new("op")
            .arg("signin")
            .arg("--raw")
            .arg(account)
            .stdout(Stdio::piped())
            .stdin(Stdio::inherit())
            .output()
            .unwrap();

        if output.status.success() {
            let token = std::str::from_utf8(&output.stdout).unwrap().trim_end();
            TokenCache {
                token: token.to_string(),
                expires_in: time::now() + 30 * 60,
            }
        } else {
            panic!(
                "err: {}",
                String::from_utf8(output.stderr.to_vec()).unwrap()
            );
        }
    }
}

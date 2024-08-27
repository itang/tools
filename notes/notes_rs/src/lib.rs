#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate notes
//!
//! add doc here

/// account
pub struct Account {
    ///name
    pub name: String,
    ///password
    pub password: String,
    ///site
    pub site: String,
}

impl Account {
    /// mask password
    pub fn mask_password(&self) -> String {
        let len = self.password.len();
        if len <= 6 {
            "*".repeat(len)
        } else {
            let head = &self.password[0..2];
            let tail = &self.password[len - 2..];
            format!("{}{}{}", head, "*".repeat(len - 4), tail)
        }
    }
}

/// Account Repository
pub struct AccountRepository;

impl AccountRepository {
    ///list
    pub fn list(&self) -> Vec<Account> {
        vec![Account {
            name: "18018786450".to_string(),
            password: "Authine@123456".to_string(),
            site: "http://39.104.227.186:8089/login?corpId=dingc55494e63f745d5335c2f4657eb6378f".into(),
        }]
    }
}

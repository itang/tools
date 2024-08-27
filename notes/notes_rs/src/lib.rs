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
        mask_text(&self.password, 2, 1)
    }
}

/// Account Repository
pub struct AccountRepository;

impl AccountRepository {
    ///list
    pub fn list(&self) -> Vec<Account> {
        vec![
            Account {
                name: "18018786450".to_string(),
                password: "Authine@123456".to_string(),
                site: "http://39.104.227.186:8089/login?corpId=dingc55494e63f745d5335c2f4657eb6378f".into(),
            },
            Account { name: "live.tang@gmail.com".to_string(), password: "Ic1".to_string(), site: "微软".into() },
            Account { name: "itang2005".to_string(), password: "Ic1".to_string(), site: "12306".into() },
            Account { name: "live.tang@gmail.com".to_string(), password: "I  - 15".to_string(), site: "weibo".into() },
            Account { name: "itang".to_string(), password: "t15".to_string(), site: "github".into() },
            Account { name: "".to_string(), password: "myc@123456".to_string(), site: "个税app".into() },
            Account { name: "root".to_string(), password: "tq521#".to_string(), site: "hk qq yun 119.28.45.29".into() },
            Account {
                name: "itang".to_string(),
                password: "it115#".to_string(),
                site: "hk qq yun 119.28.45.29".into(),
            },
            Account { name: "live.tang@gmail.com".to_string(), password: "ic1".to_string(), site: "aliyun".into() },
            Account { name: "tqibm@163.com".to_string(), password: "tq51".to_string(), site: "alipay dl".into() },
            Account { name: "tqibm@163.com".to_string(), password: "tq510".to_string(), site: "alipay zf".into() },
            Account { name: "live.tang@gmail.com".to_string(), password: "t15".to_string(), site: "github".into() },
            Account { name: "livetang".to_string(), password: "tq510".to_string(), site: "twitter.com".into() },
            Account { name: "l.t@gmail.com".to_string(), password: "i115".to_string(), site: "google".into() },
        ]
    }
}

/// Masks text with masking options.
pub(crate) fn mask_text(text: &str, num_prefix_chars: usize, num_suffix_chars: usize) -> String {
    let length = text.len();
    let suffix_index = if length > num_suffix_chars { length - num_suffix_chars } else { 0 };

    let mut masked_text = String::with_capacity(length);
    for (i, c) in text.chars().enumerate() {
        if i < num_prefix_chars || i >= suffix_index {
            masked_text.push(c);
        } else {
            masked_text.push('*');
        }
    }
    masked_text
}

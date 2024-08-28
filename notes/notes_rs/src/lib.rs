#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate notes
//!
//! add doc here

/// account
pub struct Account<'a> {
    ///public
    pub pubilc: bool,
    ///name
    pub name: &'a str,
    ///password
    pub password: &'a str,
    ///site
    pub site: &'a str,
}

impl<'a> Account<'a> {
    /// mask password
    pub fn mask_password(&self) -> String {
        mask_text(self.password, 2, 1)
    }
}

/// Account Repository
pub struct AccountRepository;

impl AccountRepository {
    ///list
    pub fn list(&self) -> Vec<Account> {
        vec![
            Account {
                pubilc: true,
                name: "18018786450",
                password: "Authine@123456",
                site: "http://39.104.227.186:8089/login?corpId=dingc55494e63f745d5335c2f4657eb6378f",
            },
            Account { pubilc: false, name: "live.tang@gmail.com", password: "Ic1", site: "微软" },
            Account { pubilc: false, name: "itang2005", password: "Ic1", site: "12306" },
            Account { pubilc: false, name: "live.tang@gmail.com", password: "I  - 15", site: "weibo" },
            Account { pubilc: false, name: "itang", password: "t15", site: "github" },
            Account { pubilc: false, name: "", password: "myc@123456", site: "个税app" },
            Account { pubilc: false, name: "root", password: "tq521#", site: "hk qq yun 119.28.45.29" },
            Account { pubilc: false, name: "itang", password: "it115#", site: "hk qq yun 119.28.45.29" },
            Account { pubilc: false, name: "live.tang@gmail.com", password: "ic1", site: "aliyun" },
            Account { pubilc: false, name: "tqibm@163.com", password: "tq51", site: "alipay dl" },
            Account { pubilc: false, name: "tqibm@163.com", password: "tq510", site: "alipay zf" },
            Account { pubilc: false, name: "live.tang@gmail.com", password: "t15", site: "github" },
            Account { pubilc: false, name: "livetang", password: "tq510", site: "twitter.com" },
            Account { pubilc: false, name: "l.t@gmail.com", password: "i115", site: "google" },
        ]
    }
}

/// Masks text with masking options.
pub(crate) fn mask_text(text: &str, num_prefix_chars: usize, num_suffix_chars: usize) -> String {
    let length = text.len();
    let suffix_index = if length > num_suffix_chars { length - num_suffix_chars } else { 0 };

    let show_middle = |len: usize, index: usize| {
        if len > 6 {
            let mid = len / 2;
            index == mid
        } else {
            false
        }
    };

    let mut masked_text = String::with_capacity(length);
    for (i, c) in text.chars().enumerate() {
        if i < num_prefix_chars || i >= suffix_index || show_middle(length, i) {
            masked_text.push(c);
        } else {
            masked_text.push('*');
        }
    }
    masked_text
}

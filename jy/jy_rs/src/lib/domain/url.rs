use std::ops::Deref;

///Url
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Url(pub String);

impl From<String> for Url {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Url {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl Url {
//     pub fn into_string(self) -> String {
//         self.0
//     }
// }

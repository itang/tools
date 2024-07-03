//! domain

mod url;

pub(crate) mod constants;

pub(crate) mod gateway;
mod jiayou_list;

pub(crate) use self::jiayou_list::JiayouList;
pub(crate) use self::url::Url;

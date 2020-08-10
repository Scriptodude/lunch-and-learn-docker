use serde::Deserialize;

mod user_info;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct UserInfo {
    pub name: String,
    pub id: usize
}
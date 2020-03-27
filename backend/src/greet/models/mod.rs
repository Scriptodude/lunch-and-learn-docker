use serde::Deserialize;

mod user_info;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub id: usize
}
use std::vec::Vec;

use super::models::UserInfo;

pub fn get_all_users() -> Vec<UserInfo> {
    return vec!(
        UserInfo{
            id: 0,
            name: "Jonathan".to_string()
        },
        UserInfo{
            id: 1,
            name: "François".to_string()
        },
        UserInfo{
            id: 2,
            name: "Guillaume".to_string()
        },
        UserInfo{
            id: 3,
            name: "Carl".to_string()
        }
    )
}

pub fn get_user_by_id(id: usize) -> UserInfo {
    return get_all_users()
        .iter()
        .find(|&u| u.id == id)
        .unwrap_or(&UserInfo{name: "Unknown".to_string(), id}).clone()
}

pub fn get_user_by_name(name: &str) -> UserInfo {
    return get_all_users()
        .iter()
        .find(|&u| u.name == name)
        .unwrap_or(&UserInfo{name: name.to_string(), id: 0}).clone()
}
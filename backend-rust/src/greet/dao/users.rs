use std::vec::Vec;
use mysql::prelude::Queryable;

use crate::database;
use super::models::UserInfo;

pub fn get_all_users() -> Vec<UserInfo> {
    let mut conn = match database::get_connection() {
        Ok(conn) => conn,
        Err(e) => {
            println!("{}", e);
            return vec!();
        }
    };

    return match conn
        .query_map(
            "SELECT id, fullname FROM users",
            |(id, fullname)| {
                UserInfo { id, name: fullname }
            }
        ) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return vec!();
        }
    }
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
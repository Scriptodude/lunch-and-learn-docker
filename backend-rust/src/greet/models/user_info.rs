use std::fmt;

impl fmt::Display for super::UserInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user {} => {}", self.id, self.name)
    }
}

impl Clone for super::UserInfo {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            id: self.id
        }
    }
}
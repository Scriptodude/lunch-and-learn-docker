use mysql;

static URL: &str = "mysql://root:root@localhost:1337/lunch_and_learn";

pub fn get_connection() -> std::result::Result<mysql::Conn, mysql::Error> {
    return mysql::Conn::new(URL);
}
use mysql;

fn get_env(name: &str) -> String {
    let env = match std::env::var(name) {
        Ok(val) => val,
        Err(e) => { println!("error fetching {}, {}", name, e); return "".to_string() }
    };

    println!("value of {} = {}", name, env);
    return env;
}

fn get_url() -> String {
    let db_url = get_env("SQL_HOST");
    let db_port = get_env("SQL_PORT");

    return format!("mysql://root:root@{}:{}/lunch_and_learn", db_url, db_port).clone();
}

pub fn get_connection() -> std::result::Result<mysql::Conn, mysql::Error> {
    return mysql::Conn::new(get_url());
}
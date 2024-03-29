use rusqlite::{Connection, Result, params};
use crate::structs::Password;

pub fn connect() -> Result<Connection> {
    let path: &str = "./src/db/storage.db";
    let conn: Connection = Connection::open(path)?;

    Ok(conn)
}

pub fn start() {
    let conn: Connection = connect().unwrap();
    conn.execute(
    "CREATE TABLE IF NOT EXISTS passwords(
            id INTEGER PRIMARY KEY,
            title VARCHAR(120) NOT NULL,
            username VARCHAR(120) NOT NULL,
            password VARCHAR(120) NOT NULL
        );", ()
    );
}

pub fn insert(title: &String, username: &String, password: &String) {
    let conn = connect().unwrap();

    conn.execute(
        "INSERT INTO passwords(title, username, password) VALUES (?1, ?2, ?3);",
        (title, username, password)
    );
}

pub fn fetch() -> Result<Vec<Password>> {
    let conn = connect().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM passwords;")?;
    let passwords_iter = stmt.query_map([], |row| {
        Ok(Password {
            id: row.get(0)?,
            title: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
        })
    })?;

    let mut all_passwords: Vec<Password> = vec![];
    for password in passwords_iter {
        all_passwords.push(password?);
    };

    Ok(all_passwords)
}

pub fn delete(id: usize) {
    let conn = connect().unwrap();

    conn.execute("DELETE FROM passwords WHERE id = ?", params![id]);
}
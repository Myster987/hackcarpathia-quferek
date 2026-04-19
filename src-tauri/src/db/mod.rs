use std::path::Path;

use rusqlite::{Connection, Row};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Użytkownik już istnieje.")]
    UserExists,
    #[error("Użytkownik nie istnieje.")]
    UserNotFound,

    #[error("Login już istnieje.")]
    LoginExists,
    #[error("Nie znaleziono loginu.")]
    LoginNotFound,
    #[error("Błąd bazy danych: {0}")]
    Database(#[from] rusqlite::Error),
}

impl serde::Serialize for Error {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Login {
    pub name: String,
    pub password: String,
}

impl From<&Row<'_>> for Login {
    fn from(value: &Row) -> Self {
        Self {
            name: value.get(0).unwrap(),
            password: value.get(1).unwrap(),
        }
    }
}

/// Create encrypted database.
pub fn create_encrypted(path: impl AsRef<Path>, password: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path)?;
    conn.execute_batch(&format!(
        "PRAGMA key = '{password}';
         CREATE TABLE IF NOT EXISTS logins (
             name          TEXT PRIMARY KEY,
             password TEXT NOT NULL
         );"
    ))?;
    Ok(conn)
}

/// Open existing database.
pub fn open_encrypted(path: impl AsRef<Path>, password: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path)?;
    conn.execute_batch(&format!("PRAGMA key = '{password}';"))?;
    Ok(conn)
}

pub fn insert_new_login(conn: &Connection, login: Login) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO logins VALUES (?1, ?2)",
        [login.name, login.password],
    )
    .map(|_| ())
    .map_err(|_| Error::LoginExists)
}

pub fn get_login(conn: &Connection, name: &str) -> Result<Login, Error> {
    conn.query_row("SELECT * FROM logins WHERE name = ?1", [name], |row| {
        Ok(Login::from(row))
    })
    .map_err(|_| Error::LoginNotFound)
}

pub fn get_all_logins(conn: &Connection) -> Result<Vec<String>, Error> {
    let mut stmt = conn.prepare("SELECT * FROM logins")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    rows.collect::<Result<Vec<_>, _>>().map_err(Error::Database)
}

pub fn delete_login(conn: &Connection, login: &str) -> Result<(), Error> {
    conn.execute("DELETE FROM logins WHERE name = ?1", [login])
        .map(|_| ())
        .map_err(|_| Error::LoginNotFound)
}

pub fn update_login(conn: &Connection, login: &str, password: &str) -> Result<(), Error> {
    conn.execute(
        "UPDATE logins SET password_hash = ?1 WHERE name = ?2",
        [password, login],
    )
    .map(|_| ())
    .map_err(|_| Error::LoginNotFound)
}

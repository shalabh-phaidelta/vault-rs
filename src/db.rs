use std::io;
use std::io::Write;
use rusqlite::{Connection, Error};

#[derive(Debug)]
pub struct ServiceInfo {
    pub id: Option<i64>,
    pub service: String,
    pub username: String,
    pub password: String, 
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        Self {
            id: None,
            service,
            username,
            password,
        }
    }
}

pub fn prompt(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
}

pub fn init_db() -> Result<Connection, Error> {
    let conn = Connection::open("password.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS PASSWORDS(
        id INTEGER PRIMARY KEY,
        service TEXT,
        username TEXT,
        password TEXT
    )", [])?;

    Ok(conn)
}

pub fn write_password_to_db(conn: &Connection, 
    service: &str,
    username: &str,
    password: &str
) -> Result<(), Error> {
    conn.execute("INSERT INTO PASSWORDS (service, username, password) VALUES(?, ?, ?)",
    &[service, username, password])?;

    Ok(())
}

pub fn read_password_from_db(conn: &Connection) -> Result<Vec<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT service, username, password FROM PASSWORDS")?;
    let entries = stmt
        .query_map([], |row| {
            Ok(ServiceInfo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>();
    entries
    // Ok(entries)
}

pub fn search_service_by_name(conn: &Connection, name: &str) -> Result<Option<ServiceInfo>, Error> {
    let mut stmt  = conn.prepare("SELECT id, service, username, password from PASSWORDS 
                WHERE service = ?")?;
    let result = stmt.query_row(&[name], |row| {
        Ok(ServiceInfo {
            id: Some(row.get(0)?),
            service: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err)
    }   
}
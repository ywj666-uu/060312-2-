pub mod schema;
pub mod queries;

use rusqlite::Connection;
use std::sync::Mutex;

pub type DbPool = actix_web::web::Data<Mutex<Connection>>;

pub fn init_db(path: &str) -> Connection {
    let conn = Connection::open(path).expect("Failed to open database");
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .expect("Failed to set pragmas");
    schema::create_tables(&conn).expect("Failed to create tables");
    conn
}

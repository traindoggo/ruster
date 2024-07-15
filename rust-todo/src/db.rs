use rusqlite::{Connection, params};

pub struct DB {
    pub connection: Connection,
}

impl DB {
    pub fn new() -> DB {
        let connection = Connection::open("dev.db").unwrap();

        connection.execute("CREATE TABLE IF NOT EXISTS todos (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            task   STRING  NOT NULL,
            status INTEGER NOT NULL
        )", params![]).unwrap();


        DB { connection }
    }
}

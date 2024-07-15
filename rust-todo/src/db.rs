use fancy::printcoln;
use rusqlite::{Connection, params};

use crate::todo::Todo;

pub struct DB {
    pub connection: Connection,
}

impl DB {
    pub fn new() -> DB {
        let connection = Connection::open("dev.db").unwrap();

        connection.execute("CREATE TABLE IF NOT EXISTS todos (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            task   TEXT    NOT NULL,
            status INTEGER NOT NULL
        )", params![]).unwrap();

        DB { connection }
    }

    pub fn show(&self) {
        let mut stmt = self.connection.prepare("SELECT id, task, status FROM todos").unwrap();

        let todos = stmt.query_map(params![], |row| {
            Ok(Todo {
                id: row.get(0).unwrap(),
                task: row.get(1).unwrap(),
                status: row.get(2).unwrap(),
            })
        }).unwrap();

        for todo in todos {
            if let Ok(todo) = todo {
                if todo.status == 0 {
                    printcoln!("{} {}", todo.id, todo.task);
                } else {
                    printcoln!("{} [s]{}", todo.id, todo.task);
                }
            }
        }
    }

    pub fn add(&self, task_name: String) {
        self.connection.execute("INSERT INTO todos (task, status) VALUES (?1, 0)",
                                params![task_name]).unwrap();
    }

    pub fn remove(&self, task_id: i32) {
        self.connection.execute("DELETE FROM todos WHERE id = ?1",
                                params![task_id]).unwrap();
    }

    pub fn edit(&self, task_id: i32, task_name: String) {
        self.connection.execute("UPDATE todos SET task = ?1 WHERE id = ?2",
                                params![task_name, task_id]).unwrap();
    }

    pub fn done(&self, task_id: i32) {
        self.connection.execute("UPDATE todos SET status = 1 WHERE id = ?1",
                                params![task_id]).unwrap();
    }

    // TODO: better rustacean way
    pub fn exists(&self, task_id: i32) -> bool {
        let mut stmt = self.connection.prepare(
            "SELECT id FROM todos WHERE id = ?1").unwrap();
        let mut rows = stmt.query(params![task_id]).unwrap();

        let mut cnt: i32 = 0;
        while let Some(_) = rows.next().unwrap() {
            cnt += 1
        }


        cnt > 0
    }
}

use fancy::printcoln;
use rusqlite::{Connection, params};

#[derive(Debug)]
pub struct Todo {
    connection: Connection,
}

#[derive(Debug)]
struct TodoRecord {
    id: i32,
    task: String,
    status: i32,
}

pub fn help() {
    printcoln!("[bold]Usage :");
    printcoln!("    [#e57373]add <task> <task> ...");
    printcoln!("    [#ffb74d]remove <task_id>");
    printcoln!("    [#fff176]edit <task_id> <task>");
    printcoln!("    [#c5e1a5]done <task_id> <task_id> ...");
}

impl Todo {
    pub fn new() -> Result<Todo, &'static str> {
        let connection = Connection::open("dev.db").unwrap();
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task STRING NOT NULL,
            status INTEGER NOT NULL)",
                params![],
            )
            .unwrap();

        Ok(Todo { connection })
    }

    pub fn show(&self) {
        let mut stmt = self
            .connection
            .prepare("SELECT id, task, status FROM todos")
            .unwrap();

        let todos = stmt
            .query_map(params![], |row| {
                Ok(TodoRecord {
                    id: row.get(0).unwrap(),
                    task: row.get(1).unwrap(),
                    status: row.get(2).unwrap(),
                })
            })
            .unwrap();

        for todo in todos {
            if let Ok(todo) = todo {
                if todo.status == 0 {
                    // undone
                    printcoln!("{} {}", todo.id, todo.task);
                } else {
                    // done
                    printcoln!("{} [s]{}", todo.id, todo.task);
                }
            }
        }
    }

    pub fn add(&self, args: &[String]) {
        for task in args {
            self.connection
                .execute(
                    "INSERT INTO todos(task, status) VALUES (?1, ?2)",
                    params![task, 0],
                )
                .unwrap();
        }
    }

    pub fn remove(&self, args: &[String]) {
        for id in args {
            self.connection
                .execute("DELETE from todos WHERE id=(?1)", params![id])
                .unwrap();
        }
    }

    pub fn edit(&self, args: &[String]) {
        self.connection
            .execute(
                "UPDATE todos SET task=(?1) WHERE id=(?2)",
                params![args[1], args[0]],
            )
            .unwrap();
    }

    pub fn done(&self, args: &[String]) {
        for id in args {
            self.connection
                .execute("UPDATE todos SET status=1 WHERE id=(?1)", params![id])
                .unwrap();
        }
    }
}

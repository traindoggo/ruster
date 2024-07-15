use std::io;
use std::io::Write;

use crate::db::DB;

pub fn get_input() -> Result<String, &'static str> {
    let mut input = String::new();

    if let Err(_) = io::stdout().flush() {
        return Err("<CLI>::get_input can't flush");
    }

    if let Err(_) = io::stdin().read_line(&mut input) {
        return Err("<CLI>::get_input can't read_line");
    }

    input = match input.trim().parse() {
        Ok(s) => s,
        Err(_) => return Err("<CLI>::get_input trim().parse() error"),
    };

    Ok(input)
}

pub struct CLI {}


impl CLI {
    pub fn show(db: &DB) {
        db.show();
    }


    pub fn add(db: &DB) {
        loop {
            print!("[add] < input task: ");

            let input_arg = get_input().unwrap();
            if input_arg == ".quit" {
                break;
            }

            db.add(input_arg);
        }
    }

    pub fn remove(db: &DB) {
        loop {
            print!("[remove] < input task: ");

            let input_arg = get_input().unwrap();
            if input_arg == ".quit" {
                break;
            }

            let input_arg = match input_arg.parse::<i32>() {
                Ok(input) => input,
                Err(_) => {
                    println!("[D:] < input task id");
                    continue;
                }
            };

            db.remove(input_arg);
        }
    }

    pub fn edit(db: &DB) {
        loop {
            print!("[edit] < input task id: ");

            let input_arg = get_input().unwrap();
            if input_arg == ".quit" {
                return;
            }

            let task_id = match input_arg.parse::<i32>() {
                Ok(input) => input,
                Err(_) => {
                    println!("[D:] < task id should be number");
                    continue;
                }
            };

            if !db.exists(task_id) {
                println!("[D:] task id {} does not exist", task_id);
                continue;
            }

            loop {
                print!("[edit] < input task name: ");
                let task_name = get_input().unwrap();
                if task_name == ".quit" {
                    return;
                }

                db.edit(task_id, task_name);
                println!();
                db.show();
            }
        }
    }


    pub fn done(db: &DB) {
        loop {
            print!("[done] < input task id: ");

            let input_arg = get_input().unwrap();
            if input_arg == ".quit" {
                break;
            }

            let task_id = match input_arg.parse::<i32>() {
                Ok(input) => input,
                Err(_) => {
                    println!("[D:] < task id should be number");
                    continue;
                }
            };

            db.done(task_id);
            println!();
        }
    }
}
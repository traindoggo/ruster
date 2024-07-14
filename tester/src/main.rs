use std::process;

use config::{ArgOption, Config};
use todo::{help, Todo};

mod config;
mod todo;

fn main() {
    // parse args
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        help();
        process::exit(1);
    });

    // load todos db (using sqlite3)
    let todo: Todo = match Todo::new() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };

    // update todos file
    match config.option {
        ArgOption::Help => help(),
        ArgOption::Show => todo.show(),
        ArgOption::Add => todo.add(&config.args),
        ArgOption::Remove => todo.remove(&config.args),
        ArgOption::Edit => todo.edit(&config.args),
        ArgOption::Done => todo.done(&config.args),
    };
}

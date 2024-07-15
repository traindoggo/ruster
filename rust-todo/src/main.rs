use std::process;

use cli::CLI;

mod cli;
mod db;
mod todo;

fn main() {
    let db = db::DB::new();

    loop {
        println!("Type: show, add, remove, edit, done and '.quit'");
        print!("[:^)] < your choice: ");

        let input_arg = cli::get_input().unwrap();

        match input_arg.as_str() {
            "show" => CLI::show(&db),
            "add" => CLI::add(&db),
            "remove" => CLI::remove(&db),
            "edit" => CLI::edit(&db),
            "done" => CLI::done(&db),
            ".quit" => {
                println!("[:^)] < GoodBye");
                process::exit(0);
            }
            _ => {}
        }

        println!();
    }
}

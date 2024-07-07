use std::fs;

fn main() {
    let current_dir = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };

    println!("current dir: {:?}", current_dir);

    for entry in fs::read_dir(current_dir).expect("something went wrong") {
        let dir = match entry {
            Ok(d) => d,
            Err(e) => panic!("{}", e),
        };

        let dirname = dir.file_name().into_string().unwrap();
        if !dirname.starts_with(".") {
            println!("{}", dirname);
        }
    }
}

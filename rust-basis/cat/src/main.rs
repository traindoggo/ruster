use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Yes");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("no file");
    println!("{}", contents);
}

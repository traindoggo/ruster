use scraper::{Html, Selector};
use std::{fs::File, io::Read};

fn main() {
    // load file
    let mut f = File::open("index.html").expect("can't open a file");
    let mut s: String = String::new();
    f.read_to_string(&mut s).expect("can't read a file");

    println!("{}", s);
}

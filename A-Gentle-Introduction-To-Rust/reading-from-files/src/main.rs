use std::{
    env,
    fs::File,
    io::{self, Read},
};

fn main() {
    result();
    //read1();
    //read2();
    read3();
}

#[allow(unused)]
fn result() {
    println!("\n>>> result");

    println!("good_or_bad(true) = {:?}", good_or_bad(true));
    println!("good_or_bad(false) = {:?}", good_or_bad(false));

    match good_or_bad(true) {
        Ok(n) => println!("Cool, I got {}", n),
        Err(e) => println!("Huh, I just got {}", e),
    }
}

fn good_or_bad(good: bool) -> Result<i32, String> {
    if good {
        Ok(42)
    } else {
        // "bad" : &str
        // "bad".to_string() : String
        Err("bad".to_string())
    }
}

#[allow(unused)]
fn read1() {
    println!("\n>>> read1");
    reading_from_files();
}

fn reading_from_files() {
    println!("\n>>> reading from files");
    let first = env::args().nth(1).expect("please supply a filename");
    let mut file = File::open(&first).expect("can't open the file");

    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");

    println!("file had {} bytes", text.len());
}

#[allow(unused)]
fn read2() {
    println!("\n>>> read2");
    let file = env::args().nth(1).expect("please supply a filename");
    let text = read_to_string(&file).expect("bad file man!");

    println!("file had {} bytes", text.len());
}

fn read_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

#[allow(unused)]
fn read3() {
    println!("\n>>> read3");
    let file = env::args().nth(1).expect("please supply a filename");
    let text = read_to_string_pretty(&file).expect("bad file man");

    println!("file had {} bytes", text.len());
}

fn read_to_string_pretty(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

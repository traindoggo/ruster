use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("can't open");
    let mut text: String = String::new();
    f.read_to_string(&mut text).expect("can't read");

    text
}

fn hash_map() {
    let text: String = read_file("poem.txt");

    let mut map = HashMap::new();
    for s in text.split(|c: char| !c.is_alphabetic()) {
        let word = s.to_lowercase();
        let mut count = map.entry(word).or_insert(0);
        *count += 1;
    }

    let mut entries: Vec<_> = map.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));

    for entry in entries.iter().take(20) {
        println!("{} {}", entry.0, entry.1);
    }
}

fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn super_set() {
    let fruit = make_set("apple orange pear orange");
    println!("{:?}", fruit);
}

fn main() {
    hash_map();
    super_set();
}

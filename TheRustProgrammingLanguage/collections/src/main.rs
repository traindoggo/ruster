use rand::Rng;
use std::collections::{HashMap, HashSet};

fn main() {
    // fn1();
    // fn2();
    // problem1();
    // problem2();
    problem3();
}

#[allow(unused)]
fn problem3() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let vs = vec!["engineer".to_string(), "sales".to_string()];

    for apartment in vs {
        map.entry(apartment)
            .or_insert_with(|| vec![])
            .push("sally".to_string());
    }

    println!("{:?}", map);
}

#[allow(unused)]
fn problem2() {
    let vs = vec![
        String::from("apple"),
        String::from("napple"),
        String::from("pencil"),
        String::from("first"),
        String::from("こんにちは、世界"),
    ];

    let vowel: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();

    for s in &vs {
        let first = s.chars().nth(0).unwrap();

        if first.is_ascii() {
            if vowel.contains(&first) {
                let f = format!("{}-hay", s);
                println!("{}", f);
            } else {
                let f = format!("{}-{}ay", &s[1..], first);
                println!("{}", f);
            }
        } else {
            println!("'{}' is not ascii...", s);
        }
    }
}

#[allow(unused)]
fn problem1() {
    let mut vs = vec![];
    let mut gen = rand::thread_rng();

    for _ in 0..gen.gen_range(100..200) {
        let num = gen.gen_range(0..20);
        vs.push(num);
    }

    vs.sort();

    println!("mean = {}", mean(&vs));
    println!("median = {}", median(&vs));
    println!("freqnet number = {:?}", frequent_number(&vs));
}

fn mean(vs: &Vec<i32>) -> f32 {
    let mut total: f32 = 0.;

    for &num in vs {
        total += num as f32;
    }

    total / vs.len() as f32
}

fn median(vs: &Vec<i32>) -> f32 {
    let size = vs.len();

    if size % 2 == 0 {
        (vs[size / 2] as f32 + vs[size / 2 + 1] as f32) / 2. as f32
    } else {
        vs[size / 2] as f32
    }
}

fn frequent_number(vs: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for &num in vs {
        let entry = map.entry(num).or_insert(0);
        *entry += 1;
    }

    let mut freq: i32 = 0;
    let mut max_cnt: i32 = 0;

    for (&num, &cnt) in &map {
        if max_cnt < cnt {
            max_cnt = cnt;
            freq = num;
        }
    }

    freq
}

#[allow(unused)]
fn fn2() {
    let text = "hello world, wonderful world!";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

#[allow(unused)]
fn fn1() {
    let teams = vec![
        String::from("blue"),
        String::from("yello"),
        String::from("red"),
    ];

    let mut map: HashMap<usize, String> = HashMap::new();

    for (i, team) in teams.into_iter().enumerate() {
        map.insert(i, team);
    }

    println!("{:?}", map);

    let vs: Vec<usize> = vec![0, 1, 2, 3];

    for i in &vs {
        let s = match map.get(i) {
            Some(num) => num,
            None => continue,
        };
        println!("{} {}", i, s);
    }
}

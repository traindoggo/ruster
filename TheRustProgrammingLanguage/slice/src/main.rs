fn main() {
    //fn1();
    fn2();
}

#[allow(unused)]
fn fn2() {
    let mut s = String::from("hello world");

    // 文字列スライス = Stringの一部への参照
    let word = first_word(&s);
    println!("{}", word);

    /*
    // error
    // word は sの参照を持っているため
    s.clear();
    println!("{}", word);
    */

    let s = "Hello World!";
    println!("{}", first_word(s));
    println!("{}", first_word(&s[..]));

    let ai = [1, 2, 3, 4, 5];
    let slice = &ai[1..3];
    println!("{:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[allow(unused)]
fn fn1() {
    println!("Hello, world!");
    let s1 = String::from("hello");
    println!("{}", s1);

    println!("{}", length(&s1));

    println!("{}", s1);
}
fn length(s: &String) -> usize {
    s.len()
}

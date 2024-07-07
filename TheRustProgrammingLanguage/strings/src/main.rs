fn main() {
    let mut string = String::from("hello");
    string.push_str(" world");
    println!("{}", string);

    let slice = "hello world";
    println!("{}", slice);

    fn1();
}

fn fn1() {
    let s = String::from("hello world");
    println!("{}", s.len());

    let mut vs: Vec<char> = vec![];
    for ch in s.chars() {
        vs.push(ch);
    }
    println!("{:?}", vs);

    let s = String::from("こんにちは、世界");
    println!("{}", s.len());

    let mut vs: Vec<char> = vec![];

    for ch in s.chars() {
        vs.push(ch);
    }
    println!("{:?}", vs);

    println!("{:?}", &s[1..4]);
}

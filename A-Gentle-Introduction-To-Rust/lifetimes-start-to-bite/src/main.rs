fn main() {
    lifetime1();
    lifetime2();
}

#[derive(Debug)]
struct A {
    // ugly notation, but necessary price
    s: &'static str,
}

fn how(i: u32) -> &'static str {
    match i {
        0 => "none",
        1 => "one",
        _ => "many",
    }
}

fn lifetime1() {
    let a = A {
        s: "hello dominant",
    };

    println!("{:?}", a);

    for i in 0..3 {
        println!("{} {}", i, how(i));
    }
}

#[derive(Debug)]
struct B<'a> {
    s: &'a str,
}

fn lifetime2() {
    let s = "I'm a little string".to_string();
    let a = B { s: &s };
    println!("{:?}", a);
}

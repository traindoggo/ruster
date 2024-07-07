fn main() {
    let s1 = String::from("hello");
    {
        // move
        // let s2 = s1;
    }

    let s1 = takes_ownership(s1);
    println!("{}", s1);
    println!("{}", calculate(&s1));

    // mutation
    let mut s1 = String::from("hello");
    mutation_string(&mut s1);
    println!("{}", s1);
}

fn takes_ownership(string: String) -> String {
    string
}

fn calculate(s: &String) -> usize {
    s.len()
}

fn mutation_string(s: &mut String) {
    s.push(',');
    s.push_str(" world");
}

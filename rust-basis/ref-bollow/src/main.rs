fn main() {
    let a = 10;
    let a_ref1 = &a;
    let a_ref2 = &a;

    println!("{} {} {} ", a, a_ref1, a_ref2);
}

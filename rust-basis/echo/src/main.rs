fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("");
        return;
    }

    println!("{}", &args[1..].join(" ").to_string());
}

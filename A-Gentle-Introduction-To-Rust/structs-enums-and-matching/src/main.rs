fn main() {
    //move_it();
    tuple1();
    zip1();
    structs();
}

fn structs() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };
    println!("{} {}", p.first_name, p.last_name);

    let p = Person::new("John", "Smith");
    println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    println!("{:?}", p);
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn zip1() {
    let names = ["ten", "hundred", "thousand"];
    let nums = [10, 100, 1000];

    for p in names.iter().zip(nums.iter()) {
        println!("{} {}", p.0, p.1);
    }
}

#[allow(unused)]
fn tuple1() {
    let t = add_mul(2.0, 10.0);
    println!("{:?}", t);
    println!("{} {}", t.0, t.1);

    let (add, mul) = t;
    println!("{} {}", t.0, t.1);
}

fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

#[allow(unused)]
fn ref1() {
    let s1 = "hello dolly".to_string();

    let mut rs1 = &s1;
    {
        let tmp = "hello dolly".to_string();
        rs1 = &tmp;
        // dropped tmp
        // so,gone back to rs1's beag heap in the sky.
    }
    // println!("{}", rs1);
}

#[allow(unused)]
fn move_it() {
    let s1 = "hello world".to_string();
    dump1(&s1);
    dump2(&s1);
    println!("{}", s1);
}

fn dump1(s: &str) {
    println!("{}", s);
}

fn dump2(s: &String) {
    println!("{}", s);
}

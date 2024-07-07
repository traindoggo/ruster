#[allow(unused_imports)]
use proconio::input;
use proconio::marker::{Bytes, Chars};

fn main() {
    method();
    //struct_enum();
    //train_and_bound();
    // generics();
    //otoshidama();
    //unreadable_string();
    //mutiple_of_9();
    //quizzes();
    //plural_form();
    //coffee();
    //a_q_c();
    //rainy_season();
    //strings();
    //new_generation_abc();
    //therefore();
    //pattern_match();
    //lifetime();
    //bit_search();
    //slice();
    //ownership_move();
    //various_distances();
    //step();
    //trapezoid_sum();
    //vector();
    //recursive();
    //crate_and_pass();
    //function();
    //mutable_variable();
    //for_expression();
    //pattern_matching();
    //ref_keyword();
}

#[allow(unused)]
fn method() {
    struct Vector(f64, f64);

    impl Vector {
        // 参照渡し move を防ぐため
        fn length(vector: &Vector) -> f64 {
            let Vector(x, y) = *vector;
            (x * x + y * y).sqrt()
        }

        // self: &Vector
        fn self_length(&self) -> f64 {
            let Vector(x, y) = *self;
            (x * x + y * y).sqrt()
        }

        fn inverse(vector: &Vector) -> Vector {
            Vector(-vector.0, -vector.1)
        }
    }

    let v = Vector(1., 1.);
    let r = Vector::length(&v);
    assert!((r - 1.41421356).abs() < 1e-6);

    let r = v.self_length();
    assert!((r - 1.41421356).abs() < 1e-6);
}

#[allow(unused)]
fn struct_enum() {
    enum Shape {
        Triangle(f64, f64, f64),
        Rectangle { height: f64, width: f64 },
        Circle { radius: f64 },
    }

    fn is_triangle(shape: &Shape) -> bool {
        if let Shape::Triangle(a, b, c) = *shape {
            true
        } else {
            false
        }
    }

    fn area(shape: &Shape) -> f64 {
        match *shape {
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.;
                let squared = s * (s - a) * (s - b) * (s - c);
                squared.sqrt()
            }
            Shape::Rectangle {
                height: h,
                width: w,
            } => h * w,
            Shape::Circle { radius } => radius * radius * std::f64::consts::PI,
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p: Point = Point { x: 10, y: 20 };
    println!("{} {}", p.x, p.y);

    let s1: Shape = Shape::Triangle(1., 2., 3.);
    let s2: Shape = Shape::Rectangle {
        height: 1.,
        width: 2.,
    };
    let s3: Shape = Shape::Circle { radius: 1. };

    assert_eq!(is_triangle(&s1), true);
    assert_eq!(is_triangle(&s2), false);

    println!("Triangle  = {}", area(&s1));
    println!("Rectangle = {}", area(&s2));
    println!("Circle    = {}", area(&s3));
}

#[allow(unused)]
fn train_and_bound() {
    // トレイト境界
    fn print<T: std::fmt::Display>(x: T) {
        println!("{}", x);
    }

    // syntax sugar
    fn print1(x: impl std::fmt::Display) {
        println!("{}", x);
    }

    fn print2<T>(x: T)
    where
        T: std::fmt::Display,
    {
        println!("{}", x);
    }

    print(10);
    print1("Hello");
    print2("hoge");
}

#[allow(unused)]
fn generics() {
    fn second_any_i32<T>(tuple: (T, i32)) -> i32 {
        tuple.1
    }

    assert_eq!(second_any_i32::<f64>((3_f64, 5)), 5);
    assert_eq!(second_any_i32::<f32>((3_f32, 5)), 5);
    assert_eq!(second_any_i32::<i64>((3_i64, 5)), 5);
    assert_eq!(second_any_i32::<bool>((true, 5)), 5);
}

#[allow(unused)]
fn otoshidama() {
    input! {
        n: i64,
        y: i64,
    }

    for big in 0..=2000 {
        for middle in 0..2000 {
            let small = n - big - middle;
            let total = big * 10000 + middle * 5000 + small * 1000;
            if small >= 0 && total == y {
                println!("{} {} {}", big, middle, small);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}

#[allow(unused)]
fn unreadable_string() {
    input! {
        s: Chars,
    }

    let size = s.len();
    let mut ans: bool = true;
    for i in 0..size {
        if i % 2 == 0 {
            if s[i].is_uppercase() {
                ans = false;
            }
        } else {
            if s[i].is_lowercase() {
                ans = false;
            }
        }
    }

    println!("{}", if ans { "Yes" } else { "No" })
}

#[allow(unused)]
fn mutiple_of_9() {
    input! {
        s: Bytes,
    }

    let mut sum: i32 = 0;

    for &ch in &s {
        //println!("{} - {} = {}", ch, b'0', ch - b'0');
        sum += (ch - b'0') as i32;
    }

    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(unused)]
fn quizzes() {
    input! {
        _: usize,
        mut score: i32,
        s: Chars,
    }

    for &ch in &s {
        if ch == 'x' {
            score = std::cmp::max(score - 1, 0);
        } else {
            score += 1;
        }
    }

    println!("{}", score);
}

#[allow(unused)]
fn plural_form() {
    input! {
        s: Chars,
    }

    let size = s.len();
    let form = {
        if s[size - 1] == 's' {
            "es"
        } else {
            "s"
        }
    };

    for ch in &s {
        print!("{}", ch);
    }
    println!("{}", form);
}

#[allow(unused)]
fn coffee() {
    input! {
        s: Chars,
    }

    let ans = { s[2] == s[3] && s[4] == s[5] };
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(unused)]
fn a_q_c() {
    input! {
        s: String,
    }

    let ans = if s == "ABC" { "ARC" } else { "ABC" };
    println!("{}", ans);
}

#[allow(unused)]
fn rainy_season() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    let mut cnt = 0;
    for ch in s {
        match ch {
            'R' => cnt += 1,
            'S' => cnt = 0,
            _ => unreachable!(),
        }
        ans = ans.max(cnt);
    }
    println!("{}", ans);
}

#[allow(unused)]
fn strings() {
    println!(">>> strings");
    {
        // String
        let s: String;
        let s = String::new();

        // string slice
        let slice: &str = &s;
        // 文字列リテラル はそもそも文字列
        let slice = "Hello";
    }

    {
        // slice to String
        let string = "Hello".to_string();
        let string = String::from("Hello");
    }

    {
        let s = "Hello";
        for c in s.chars() {
            print!("{}", c);
        }
    }
    println!();
}

#[allow(unused)]
fn new_generation_abc() {
    input! {
        n: i32,
    }

    let ans = match n {
        0..=125 => 4,
        126..=211 => 6,
        212..=214 => 8,
        _ => unreachable!(),
    };
    println!("{}", ans);
}

#[allow(unused)]
fn therefore() {
    input! {
        n: i32,
    }

    let ans = match n % 10 {
        2 | 4 | 5 | 7 | 9 => "hon",
        0 | 1 | 6 | 8 => "pon",
        3 => "bon",
        _ => unreachable!(),
    };
    println!("{}", ans);
}

#[allow(unused)]
fn pattern_match() {
    let tuple: (i32, f64) = (1, 2.);
    if let (x, 2.) = tuple {
        println!("{} 2.0", x);
    } else if let (1, y) = tuple {
        println!("{}", y);
    } else {
        println!("not match")
    }

    let num = 10;
    let ans = match num {
        0..=5 => "small",
        0..=10 => "huge",
        _ => "not match",
    };
    println!("{}", ans);
}

#[allow(unused)]
fn lifetime() {
    println!("\n>>> lifetime");

    fn sum(v: Vec<i32>) -> i32 {
        let mut ret = 0;
        for &n in &v {
            ret += n;
        }
        ret
    }

    fn sum_s(v: &Vec<i32>) -> i32 {
        let mut ret = 0;
        for &n in v {
            ret += n;
        }
        ret
    }

    fn double(x: &mut i32) {
        *x *= 2;
    }

    let vector = vec![10, 20, 30, 40];
    println!("{}", sum_s(&vector));

    // died s lifetime
    println!("{}", sum(vector));
    //println!("{:?}", vector);

    let mut s = 0;
    println!("{}", s);
    double(&mut s);
    println!("{}", s);
}

#[allow(unused)]
fn bit_search() {
    let array = [0, 1, 2, 3, 4, 5, 6];

    let size = array.len() - 3;
    for bit in 0..(1 << size) {
        print!("bit : {} <", bit);
        for i in 0..size as i32 {
            if bit & (1 << i) != 0 {
                print!("{},", i);
            }
        }
        println!(">");
    }
}

#[allow(unused)]
fn slice() {
    // slice
    let mut ref_slice: &[i32];

    let array = [1, 2, 3];
    ref_slice = &array;
    println!("&ref = {:p}", ref_slice);
    println!(" ref = {:?}", ref_slice);

    let vector = vec![4, 5, 6];
    ref_slice = &vector;
    println!("&ref = {:p}", ref_slice);
    println!(" ref = {:?}", ref_slice);

    let array = [0, 1, 2, 3, 4, 5, 6];
    // slice は特別な型 常に参照の形で使う必要がある
    println!("array = {:?}", &array[1..4]);
}

#[allow(unused)]
fn ownership_move() {
    {
        let ref_elem;
        {
            let vector = vec![10, 20, 30];
            {
                let ref_entire = &vector;
            }
            ref_elem = &vector[1];
        }
    }

    {
        let vector = vec![10, 20, 30];
        // moved ownership
        let moved = vector;
        //println!("{:?}", vector);
        println!("{:?}", moved);
    }

    {
        let test_scores = vec![1, 3, 2];
        let total_score = sum(test_scores);
        assert_eq!(total_score, 6);

        // use of moved value
        // test_scores;
    }

    fn sum(vector: Vec<i32>) -> i32 {
        let mut ret = 0;
        for elem in &vector {
            ret += elem;
        }
        ret
    }
}

#[allow(unused)]
fn various_distances() {
    input! {
        n: usize,
        xi : [f64; n],
    }

    let manhattan: f64 = {
        let mut dist: f64 = 0.;
        for &x in &xi {
            dist += x.abs();
        }
        dist
    };

    let euclidean: f64 = {
        let mut dist: f64 = 0.;
        for &x in &xi {
            dist += x.powf(2.0);
        }
        dist.sqrt()
    };

    let chevshev: f64 = {
        let mut dist: f64 = 0.;
        for &x in &xi {
            dist = x.abs().max(dist);
        }
        dist
    };

    println!("{}", manhattan);
    println!("{}", euclidean);
    println!("{}", chevshev);
}

#[allow(unused)]
fn step() {
    input! {
        n: usize,
        ai: [i64; n],
    }

    let mut ans = 0;
    let mut current_height = ai[0];

    for &a in &ai {
        current_height = current_height.max(a);
        ans += current_height - a;
    }

    println!("{}", ans);
}

#[allow(unused)]
fn trapezoid_sum() {
    input! {
        n: usize,
        v: [(i64, i64); n],
    }

    let mut sum: i64 = 0;

    for &(l, r) in &v {
        let size: i64 = r - l;
        sum += {
            let n: i64 = r - l + 1;
            let s = (l + r) * n / 2;
            s
        };
    }

    println!("{}", sum);
}

#[allow(unused)]
fn vector() {
    println!("\n>>> vector");

    let v1: Vec<i32> = Vec::<i32>::new();
    println!("{:?}", v1);

    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    let v3 = vec![1; 3];
    println!("{:?}", v3);

    {
        let mut sum = 0;
        for num in &v3 {
            sum += num;
        }
        assert_eq!(sum, 6);
    }

    {
        let mut ans = 100;
        let ai = vec![40, 50, 20, 60, 30];
        for &i in &ai {
            ans = ans.min(i);
        }
        assert_eq!(ans, 20);
    }
}

#[allow(unused)]
fn recursive() {
    println!("\n>>> crate_and_pass");

    fn fact(n: i32) -> i32 {
        if n <= 0 {
            1
        } else {
            fact(n - 1) * n
        }
    }
    println!("{}", fact(5));

    fn mygcd(n: i32, m: i32) -> i32 {
        if m == 0 {
            n
        } else {
            mygcd(m, n % m)
        }
    }
    println!("{}, {}", mygcd(18, 30), mygcd(45, 30));
}

#[allow(unused)]
fn crate_and_pass() {
    println!("\n>>> crate_and_pass");
    println!("{}", rand::random::<i32>());
    println!("{}", num::integer::gcd(18, 30));

    println!("{}", std::cmp::max(2_i64, 5_i64));
    println!("{}", std::cmp::min(2_usize, 5_usize));
}

#[allow(unused)]
fn function() {
    println!("\n>>> function");

    let value = {
        let mut n = 1;

        for i in 1..=5 {
            n *= i;
        }

        n
    };
    assert_eq!(value, 120);
}

#[allow(unused)]
fn mutable_variable() {
    println!("\n>>> mutable variable");

    let mut mutable: i32;
    mutable = 30;
    assert_eq!(30, mutable);

    mutable += 10;
    assert_eq!(40, mutable);

    {
        let array = [30, 20, 10];
        let mut sum = 0;
        for num in &array {
            sum += num;
        }
        assert_eq!(60, sum);
    }

    {
        let mut hoge = 10;
        let reference = &hoge;
        println!("{}", reference);
    }

    {
        let mut hoge = 10;

        // 可変参照
        let reference = &mut hoge;
        // dereference
        *reference = 30;
        assert_eq!(*reference, 30);

        *reference = 100;
        assert_eq!(*reference, 100);

        assert_eq!(hoge, 100);
    }
}

#[allow(unused)]
fn ref_keyword() {
    println!("\n>>> ref");
    let carbon = (1, 2);
    let (ref left, ref right) = carbon;

    println!("{:?}", carbon);
    assert_eq!(*left, 1);
    assert_eq!(*right, 2);
}

#[allow(unused)]
fn for_expression() {
    println!("\n>>> for expression");

    let primes = [2, 3, 5, 7];

    for p in &primes {
        println!("{}", p);
    }
}

#[allow(unused)]
fn pattern_matching() {
    println!("\n>>> pattern matching");
    let elements: [(i32, f64); 5] = [(6, 12.0), (7, 14.0), (8, 16.0), (15, 31.0), (16, 32.1)];
    for &(number, weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }
}

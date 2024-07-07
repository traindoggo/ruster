fn main() {
    //for_var_mut();
    fun1();
}

#[allow(dead_code)]
fn fun1() {
    println!("*** fun1");

    let res = sqr(2.0);
    println!("square is {}", res);

    let res = abs(-2.0);
    println!("abs is {}", res);

    let res = factorial(6);
    println!("factorial is {}", res);

    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&40);
    println!("{}, {}", res1, res2);

    let mut res = 0.0;
    modifies(&mut res);
    println!("modifies : {}", res);
}

// values can also be passed by reference.
// A reference is created by & and dereferenced by *
fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn factorial(n: u64) -> u64 {
    if n <= 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn sqr(x: f64) -> f64 {
    return x * x;
}

#[allow(dead_code)]
fn for_var_mut() {
    println!("*** for var mut");
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even: {}", i)
        } else {
            println!("odd : {}", i)
        }
    }
    println!();

    for i in 0..5 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{} {}", even_odd, i);
    }
    println!();

    // Immutable
    // A variable that cannont vary?
    // `let` variables by default can only be assigned a value
    // when declared.
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}\n", sum);

    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {}", sum);
}

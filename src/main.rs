use std::io;

fn main() {
    // prints the requested number of fibonacci sequence elements
    println!("How many fibonacci numbers?");

    let mut i = get_num();
    let mut n = 1;

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    while n <= i {
        println!("fib {}: {}", n, a);
        c = a + b;
        a = b;
        b = c;

        n = n + 1;
    }
}

fn get_num() -> i32 {
    // gets user input and returns it as i32
    let mut num = String::new();

    io::stdin().read_line(&mut num)
        .expect("failed to read input.");

    let mut num: i32 = num.trim().parse().expect("invalid input");

    num
}

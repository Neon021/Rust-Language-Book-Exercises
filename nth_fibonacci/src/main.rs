use std::io;

fn main() {
    println!("Nth fibonacci number, please enter n");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Please enter a valid integer value");

    let n: u32 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => panic!("Please enter a positive value")
    };

    let res = fibonacci(n);

    println!("{n}th fibonacci value is:\r\n{res}");
}
fn fibonacci(n: u32) -> u32{
    if n <= 1{
        return n;
    }
    else{
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

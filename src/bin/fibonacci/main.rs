use std::io;

fn main() {
    println!("Generate the nth Fibonacci number. Enter the value for n:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not convert input to an unsigned int."),
    };

    let result = fibonacci(n, 0, 1);
    println!("Nth Fibonacci number is {result}");
}

fn fibonacci(n: u32, previous_value: u32, current_value: u32) -> u32 {
    if n == 1 {
        current_value
    } else {
        fibonacci(n - 1, current_value, current_value + previous_value)
    }
}

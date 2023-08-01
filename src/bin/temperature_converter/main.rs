use std::io;
use std::os::raw::c_float;

fn main() {
    println!("To convert from Fahrenheit, enter 'F'. To convert from Celsius, enter 'C'.");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line.");

    let unit = unit.trim().to_uppercase();

    println!("Enter the value to convert:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let input: c_float = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not convert input to a number."),
    };

    let result = if unit == "C" {
        input * 9.0 / 5.0 + 32.0
    } else {
        (input - 32.0) * (5.0 / 9.0)
    };

    println!("The result is {result}")
}

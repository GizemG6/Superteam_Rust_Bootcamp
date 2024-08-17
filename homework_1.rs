use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num1: f64 = input.trim().parse().expect("Please type a number");

    input.clear();
    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation = input.trim().to_string();

    input.clear();
    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num2: f64 = input.trim().parse().expect("Please type a number");

    let op = match operation.as_str() {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = calculate(op);
    println!("The result is: {}", result);
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

use std::io;
use colored::*;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    let mut input = String::new();

    // first number
    println!("{}", "Enter the first Number!".yellow());
    io::stdin().read_line(&mut input).expect("Failed to read the line.");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");
    // println!("Num is:{}", num1);
    input.clear();

    // second number 
    println!("{}", "Enter the second Number!".yellow());
    io::stdin().read_line(&mut input).expect("Failed to read the line.");
    let num2: f64 = input.trim().parse().expect("hiii");
    // println!("Num2 is: {}", num2);
    input.clear();
    
    // operation
    println!("{}", "Choose the operation's (+, -, *, /)".yellow());
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation = input.trim();
    // println!("The choosed operation is: {}", operation);
    
    
    


    let operation = match operation {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("{}","Invalid operation, choose from the above four!!".red());
            return;
        }
    };


    let result = calculate(operation);
    println!("The result is: {}", result);
    
}

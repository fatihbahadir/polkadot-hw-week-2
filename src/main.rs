use std::io;

fn main() {
    // Get the first number from the user
    println!("Enter the first number");
    let first_number: f64 = read_input_as_float();

    // Get the operation to perform from the user
    println!("Enter the operation you want to perform: 
    1- Add
    2- Subtract
    3- Multiply 
    4- Divide");
    let operation_input = get_input();   

    // Get the second number from the user
    println!("Enter the second number");
    let second_number: f64 = read_input_as_float();

    // Match the operation input and create the corresponding Operation enum variant
    let operation = match operation_input.trim(){
        "1" => Operation::Add(first_number,second_number),
        "2" => Operation::Subtract(first_number, second_number),
        "3" => Operation::Multiply(first_number, second_number),
        "4" => Operation::Divide(first_number, second_number),
        _ => panic!("Operation is not allowed!")
    };

    // Calculate the result based on the chosen operation
    let result: f64 = calculate(operation);
    
    println!("The result is: {}", result);
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

// Perform the calculation based on the provided operation
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => {
            a + b
        }
        Operation::Subtract(a, b) => {
            a - b 
        }
        Operation::Multiply(a, b) => {
            a * b
        }
        Operation::Divide(a, b) => {
            a / b
        }
    }
}

// Read user input as a floating-point number
fn read_input_as_float() -> f64{
    let input: String = get_input();
    convert_to_float(&input)
}

// Get input from the user as a string
fn get_input() -> String{
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Cannot read the value");
    input
}

// Convert a string to a floating-point number
fn convert_to_float(input: &String) -> f64 {
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            0.0
        }
    }
}
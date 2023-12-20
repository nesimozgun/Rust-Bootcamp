use std::io; // use io for input-output processes

//create an enum with its variants for 4 operations
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// create a function that uses pattern matching and performs operations based on Operation enum
fn calculate(opr: Operation) -> Result<f64, &'static str> { 
    match opr {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                return Err("Cannot divide by zero");
            }
            Ok(a / b)
        },
    }
}

fn main() {
    let mut input = String::new(); // create a variable to assign inputs to it

    // get the input and parse the f64 number value
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Invalid input");
    input.clear(); //clear input value

    // get the operation symbol as a character input
    println!("Type the operator (+: Add, -: Subtract, *: Multiply, /: Divide): ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operator: char = input.trim().chars().next().expect("Invalid input");
    input.clear(); // clear input value

    //  get the second input and parse the f64 number value
    println!("Enter the second number: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Invalid input");
    input.clear(); // clear input value

    // match symbols with operations
    let opr = match operator {
        '+' => Some(Operation::Add(a, b)),
        '-' => Some(Operation::Subtract(a, b)),
        '*' => Some(Operation::Multiply(a, b)),
        '/' => Some(Operation::Divide(a, b)),
        _ => None,
    };

    // check if the input matches any operation and print the result is okay
    match opr {
        Some(opr) => {
            if let Ok(res) = calculate(opr) {
                println!("Result: {}", res);
            } else {
                println!("Invalid operation");
            }
        }
        None => println!("Invalid operation"),
    }
}
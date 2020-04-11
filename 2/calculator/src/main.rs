use std::io::{
    stdin, 
    stdout, 
    Write // Flush
};

use std::collections::HashMap;

fn get_number(msg: &str) -> u32 {
    let input = get_string(msg);
    input.parse::<u32>().expect("Not a number")
}

fn get_string(msg: &str) -> String{
   print!("{}\n>\t", msg);
   stdout().flush().expect("Flush failed");
   let mut input = String::new();
   stdin().read_line(&mut input).expect("Failed to read line"); 
   String::from(input.trim())
}

fn main() {
    println!("Hello and Welcome to the Calculator");
    let n1 = get_number("Enter the first number");
    let n2 = get_number("Enter the second number");
    let operation = get_string("Enter the operation symbol");

    type Callback = fn(&u32,&u32)->u32;
    let operations : HashMap<String, Callback> = [ 
        ("+".to_string(), (|&a, &b| a + b) as Callback), // closures have their own types so need to force to  function pointer
        ("-".to_string(), (|&a, &b| a - b) as Callback),
        ("*".to_string(), (|&a, &b| a * b) as Callback),
        ("/".to_string(), (|&a, &b| a / b) as Callback) ].iter().cloned().collect();

    match operations.get(&operation) {
        Some(&cb) => println!("{} {} {} = {}", n1, operation, n2, cb(&n1, &n2)),
        None => eprintln!("{} is not a valid operation", operation),
    }
}

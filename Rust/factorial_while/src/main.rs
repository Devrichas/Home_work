use std::io::{self, Write};

fn convert_to_int(data_input: & String) -> i32 {
    let number = data_input.trim().parse::<i32>().unwrap();
    return number;
}

fn main() {
    let mut n = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let mut factorial = 1;
    let mut n_int = convert_to_int(&n);

    // Calculate factorial using a while loop
    while n_int > 1 {
        factorial = factorial * n_int;
        n_int = n_int - 1;
    }

    println!("The factorial is: {}", factorial);
}

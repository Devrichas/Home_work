use std::io::{self, Write};

fn convert_to_int(data_input: & String) -> i32 {
    let number = data_input.trim().parse::<i32>().unwrap();
    return number;
}


fn main() {
    let mut data_input = String::new();
    print!("Digite um número para calcular o fatorial: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut data_input).expect("Failed to read input");
    let number = convert_to_int(&data_input);
    let mut factorial = 1;
    for i in 1..=number {
        factorial *= i;
    }
    println!("Factorial of {} is {}", number, factorial);
}

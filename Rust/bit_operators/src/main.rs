use std::io::{self, Write};
use std::ops::Shl;
use std::ops::Shr;

fn matade(num1: i32) -> i32 {
    return num1.shr(1);
}

fn dobro(num1: i32) -> i32 {
    return num1.shl(1);
}

fn main() {
    let mut number1 = String::new();
    print!("Digite um numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number1).expect("Failed to read line");

    let num1 = number1.trim().parse::<i32>().unwrap();

    println!("O dobro de {} e {}", num1, dobro(num1));
    println!("A metade de {} e {}", num1, matade(num1));
}

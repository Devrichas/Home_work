use std::io::{self, Write};

fn convert_to_int(data_input: & String) -> i32 {
    let number = data_input.trim().parse::<i32>().unwrap();
    return number;
}

fn main() {
    let mut number1 = String::new();
    print!("Digite o primeiro numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number1).expect("Falha ao ler o numero");
    let mut number1 = convert_to_int(&number1);
    
    let mut number2 = String::new();
    print!("Digite o segundo numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number2).expect("Falha ao ler o numero");
    let mut number2 = convert_to_int(&number2);

    while number2 != 0 {
        let temp = number2;
        number2 = number1 % number2;
        number1 = temp;
    }
    println!("O maior divisor comum é: {}", number1);
}

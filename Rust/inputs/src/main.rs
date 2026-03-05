use std::io::{self, Write};

fn convert_to_int(data_input: & String) -> i32 {
    let number = data_input.trim().parse::<i32>().unwrap();
    return number;
}



fn main() {
    let mut number1 = String::new();
    print!("Digite o primeiro numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number1).expect("Failed to read line");

    
    let mut number2 = String::new();
    print!("Digite o segundo numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number2).expect("Failed to read line");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("O numero {} e maior que {}", number1, number2);
    }

    else if convert_to_int(&number1) < convert_to_int(&number2) {
        println!("O numero {} e menor que {}", number1, number2);
    }

    else {
        println!("Os numeros sao iguais");
    }

}

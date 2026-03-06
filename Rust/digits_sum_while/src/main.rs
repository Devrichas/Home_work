use std::io::{self, Write};

fn convert_to_int(data_input: & String) -> i32 {
    let number = data_input.trim().parse::<i32>().unwrap();
    return number;
}

//function to sum the digits of a number
fn main() {
    let mut soma = 0;
    let mut valor_entrada = String::new();
    print!("Digite o primeiro numero: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut valor_entrada).expect("Falha ao ler o numero");
    let mut number1_i32 = convert_to_int(&valor_entrada);
    
    //loop that goes through the digits of the number and sum them by dividing the number by 10 and getting the rest of the division, when the number is 0 the loop ends
    while number1_i32 != 0 {
        let resultado = number1_i32 %10;
        soma = soma + resultado;
        number1_i32 = number1_i32 / 10;
    }
    
    println!("A soma dos digitos é: {}", soma);
}


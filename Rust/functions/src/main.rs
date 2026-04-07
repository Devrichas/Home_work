use std::io::{self, Write};

fn convert_to_int(data_input: & String) -> i32 {
    let number = data_input.trim().parse::<i32>().unwrap();
    return number;
}

fn dobro(num: i32) -> i32{
    return 2*num;
}

fn maior(num1: i32, num2: i32) -> i32 {
    if num1 > num2 {
        return num1;
    }
    else if num1 < num2 {
        return num2;
    }
    else {
        return num1; // or num2, since they are equal
    }
}

fn func_exemplo(param1: f32, param2: i128) -> f32 {
    println!("Essa funcao retorna um vlor do tipo f32");
    //a funcao retona o valor 10 como um numero do tipo f32, nao e necessario passar nenhum parametro para a funcao
    10 as f32
    // retornaria o valor de x, o param2 e convertido para f32 para que a operacao seja realizada
    let x = 10.1 * param1 + param2 as f32;
    x
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

    let num1 = convert_to_int(&number1);
    let num2 = convert_to_int(&number2);

    println!("O maior numero entre {} e {} é: {}", num1, num2, maior(num1, num2));
}


// fn main() {
//     let mut number1 = String::new();
//     print!("Digite o numero: ");
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut number1).expect("Failed to read line");
//     let num = convert_to_int(&number1);
//     println!("O dobro do numero é: {}", dobro(num));
// }


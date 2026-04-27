fn primo(num: i32) -> bool {
    
    let limite = (num as f64).sqrt() as i32 + 1;

    if num <= 1{
        return false;
    }

    for i in 2..limite{
        if num % i == 0{
            return false;
        }
    }

    return true;

}

fn main() {
    let numero = 28;
    let resultado = primo(numero);
    println!("O número {} é primo? {}", numero, resultado);
}
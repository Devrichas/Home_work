fn media(vector: Vec<f64>) -> f64 {
    let sum: f64 = vector.iter().sum();
    let count = vector.len() as f64;
    return sum / count
}

fn main(){
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = media(numbers);
    println!("A média é: {}", result);
}
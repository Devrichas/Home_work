fn soma(vector : Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in vector {
        sum += i;
    }
    return sum
}

fn main() {
    let vector = vec![1, 2, 3, 4, 5];
    println!("A soma dos elementos é: {}", soma(vector));
}

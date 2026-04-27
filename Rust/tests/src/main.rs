fn count(num: i32) {
    for i in 1..=num{
        println!("Current number: {}", i);
    }
}

fn count_down(mut num:i32){
    while num > 0 {
        println!("Current number: {}", num);
        num -= 1;
    }
}

fn main() {
    count(10);
    count_down(10);
}

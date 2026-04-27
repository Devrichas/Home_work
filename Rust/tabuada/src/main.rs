fn tabuada(num: i32){
    for i in 1..=10{
        println!("{} x {} = {}", num, i, num * i);
    }
}


fn main(){
    tabuada(7);
}
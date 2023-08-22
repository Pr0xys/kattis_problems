use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("error!");
    let numbers:Vec<i32> = numbers.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    if numbers[0] > numbers[1]{
        println!("1");
    }else{
        println!("0");
    }
}



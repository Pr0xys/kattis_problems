use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let _ignore_this: i32 = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("error");
    let numbers: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut output: i32 = 0;
    for i in 0 .. numbers.len(){
        output += numbers[i];
    }
    println!("{}",output);
}

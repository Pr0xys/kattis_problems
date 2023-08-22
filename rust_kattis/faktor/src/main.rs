use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let a: i32 = input[0];
    let i: i32 = input[1];

    let bribes: i32  = a * (i - 1) + 1;
    println!("{}", bribes);
}

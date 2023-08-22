use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error!");
    let input:Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let output = 2 * input[1] - input[0];
    println!("{}",output);
}


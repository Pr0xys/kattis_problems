use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    input.clear();
    io::stdin().read_line(&mut input).expect("error");
    let first: HashSet<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    input.clear();
    io::stdin().read_line(&mut input).expect("error");
    let second: HashSet<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let output: Vec<&i32> = first.difference(&second).collect::<Vec<&i32>>();
    println!("{}", output[0]);
}

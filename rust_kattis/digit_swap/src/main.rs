use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<char> = input.chars().collect();
    println!("{}{}", input[1], input[0]);
}

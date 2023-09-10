use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let input: String = input.trim().chars().rev().collect();
    println!("{}",input);
}

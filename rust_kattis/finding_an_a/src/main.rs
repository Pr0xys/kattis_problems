use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input_index = input.trim().chars().position(|ch| ch == 'a').unwrap_or(0);
    let output = &input[input_index..];
    println!("{}", output);
}



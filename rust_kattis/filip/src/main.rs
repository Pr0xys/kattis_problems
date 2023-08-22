use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let input: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x
             .chars()
             .rev()
             .collect::<String>()
             .parse::<i32>()
             .unwrap())
        .collect();

    if input[0] > input[1]{
        println!("{}", input[0]);
    } else {
        println!("{}", input[1]);
    }
}

use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input:Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    match input[0].cmp(&input[1]) {
        Ordering::Less => println!("{} {}", input[0],input[1]),
        Ordering::Greater => println!("{} {}", input[1], input[0]),
        Ordering::Equal => println!("{} {}", input[0], input[0]),
    }
}

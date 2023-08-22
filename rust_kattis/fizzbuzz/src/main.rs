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
             .parse()
             .unwrap())
        .collect();
    for i in 1 .. input[2] +1 {
        match i {
            _ if i % input[0] == 0 && i % input[1] == 0 => println!("FizzBuzz"),
            _ if i % input[0] == 0 => println!("Fizz"),
            _ if i % input[1] == 0 => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}

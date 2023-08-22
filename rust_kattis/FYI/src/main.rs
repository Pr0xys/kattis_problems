use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error!");
    let input: i32 =input[0..3].trim().parse().unwrap();

    match input {
        555 => println!("1"),
        _ => println!("0"),

    }
}

use std::io;

fn main() {
    let mut input = String::new();
    let mut input_two = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error");
    io::stdin()
        .read_line(&mut input_two)
        .expect("error");

    let input: Vec<_> = input
        .trim()
        .split("|")
        .collect();
    
    let input_two: Vec<_> = input_two
        .trim()
        .split("|")
        .collect();
    
    println!("{}{} {}{}", input[0],input_two[0],input[1],input_two[1]);
}

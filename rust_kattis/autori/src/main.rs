use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<char> = input.chars().collect();
    let mut output = String::new();
    for i in input {
        if i.is_uppercase() {
            output.push(i)
        }
    }
    println!("{}",output);
}

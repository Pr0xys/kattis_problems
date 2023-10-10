use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");

    let mut output = String::new();
    let mut prev = ' ';

    input.trim().chars().for_each(|current| {
        if current != prev {
            output.push_str(&current.to_string());
        }
        prev = current;
    });

    println!("{}", output);
}

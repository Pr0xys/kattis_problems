use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let mut output = String::new();
    let mut prev_char: Option<char> = None;

    for ch in input.chars() {
        if prev_char != Some(ch) {
            output.push(ch);
            prev_char = Some(ch);
        }
    }

    println!("{}", output);
}

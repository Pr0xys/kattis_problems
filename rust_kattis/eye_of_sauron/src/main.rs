use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");

    let mut first_chars: Vec<char> = Vec::<char>::new();

    let mut second_chars: Vec<char> = Vec::<char>::new();

    let mut found_parenthesis = false;

    for ch in input.trim().chars() {
        if ch == '(' {
            found_parenthesis = true;
            first_chars.push(ch);
            continue;
        }

        if found_parenthesis {
            second_chars.push(ch);
        } else {
            first_chars.push(ch);
        }
    }

    if first_chars.len() == second_chars.len(){
        println!("correct");
    } else {
        println!("fix");
    }
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let mut prev_char: Option<char> = None;
    let mut counter = 0;

    for i in input.chars(){
        if i == 's' && prev_char == Some('s'){
            println!("hiss");
            counter += 1;
            break;
        }
        prev_char = Some(i);
    }
    if counter == 0 {    
        println!("no hiss");
    }
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let mut input: String = input.trim().parse().unwrap();
    input.pop();
    for _ in 0 .. input.chars().count()-1{
        input.push_str("e");
    }
    
    input.push_str("y");
    println!("{}", input);

}



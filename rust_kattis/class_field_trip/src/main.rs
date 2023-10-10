use std::io;
use std::iter::Iterator;


fn main() {
    let mut input = String::new();
    let mut input_two = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error");
    
    io::stdin()
        .read_line(&mut input_two)
        .expect("error");
    
    let output = format!("{}{}", input.trim(), input_two.trim());
    let mut output:Vec<char> = output.chars().collect();
    output.sort();
    let output: String = output.into_iter().collect();
							
    println!("{}", output);
}

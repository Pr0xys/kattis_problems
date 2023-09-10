use std::io; 

fn main() {
    let mut output: i32 = 0;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");
    output += input
        .trim()
        .parse::<i32>()
        .unwrap();
    
    input
        .clear();
    
    io::stdin()
        .read_line(&mut input)
        .expect("input error");
    
    output += input
        .trim()
        .parse::<i32>()
        .unwrap();

    println!("{}",output);
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");

    let cost: f64 = input
        .trim()
        .parse()
        .unwrap();
    
    input
        .clear();
    
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    
    let n: i32 = input
        .trim()
        .parse()
        .unwrap();
    
    let mut output: f64 = 0.0;

    for _ in 0 .. n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("error");
        let new_input: Vec<f64> = input
            .trim()
            .split_whitespace()
            .map(|x| x
                 .parse()
                 .unwrap())
            .collect();
        let square: f64 = new_input[0] * new_input[1];
        
        output += cost * square
    }
    
    println!("{:.8}", output);
}

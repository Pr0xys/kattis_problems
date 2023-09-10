use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    
    let output:usize = input
        .chars()
        .filter(|&x| x == 'u')
        .count();

    println!("{}",output);
}

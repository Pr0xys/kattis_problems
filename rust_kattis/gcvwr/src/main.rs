use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let first_line: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x
             .parse()
             .unwrap()
             )
        .collect();

    input.clear();
    
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    
    let second_line: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x
             .parse()
             .unwrap()
             )
        .collect();

    let total_items: i32 = second_line.iter().fold(0, |acc, num| acc + num);
    
    let output: i32 = (((first_line[0] as f64 - first_line[1] as f64) * 0.90) - total_items as f64) as i32;

    println!("{}",output);
}

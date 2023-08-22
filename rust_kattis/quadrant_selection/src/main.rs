use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let a: i32 = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).expect("Error");
    let b: i32 = input.trim().parse().unwrap();
    
    let pair = (a,b);
    match pair {
        (a, b) if a < 0 && b < 0 => {println!("3")},
        (a, _) if a < 0 => {println!("2")},
        (_, b) if b < 0 => {println!("4")},
        _ => {println!("1")},
    }
}

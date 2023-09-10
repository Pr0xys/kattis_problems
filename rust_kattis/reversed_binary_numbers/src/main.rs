use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let n:i32 = input
        .trim()
        .parse()
        .unwrap();
    let binary_str = format!("{:b}", n)
        .to_string();
    let reversed_binary_str: String = binary_str
        .chars()
        .rev()
        .collect();
    let reversed_n = isize::from_str_radix(&reversed_binary_str, 2)
        .expect("error");

    println!("{}", reversed_n);
}

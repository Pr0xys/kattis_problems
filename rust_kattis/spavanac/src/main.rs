use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    if input[1] - 45 < 0 {
        if input[0] - 1 < 0 {
            println!("{} {}", 23, input[1] - 45 + 60);
        } else {
            println!("{} {}", input[0] - 1, input[1] - 45 + 60);
        }
    } else {
        println!("{} {}", input[0], input[1] - 45);
    }
}



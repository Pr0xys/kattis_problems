use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    //let input: i32 = input.trim().parse().unwrap();
    // match input {
    //    _ if input % 2 == 1 => {println!("first")},
    //    _ if input % 2 == 0 => {println!("second")},
    //    _ => todo!()
    //}

    let res = input
        .trim()
        .chars()
        .enumerate()
        .filter(|(x, y)| y != input[y + 1])
        .collect();
}

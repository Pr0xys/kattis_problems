use std::io;

fn main() {
    let mut input_one = String::new();
    let mut input_two = String::new();

    io::stdin()
        .read_line(&mut input_one)
        .expect("error");
    io::stdin()
        .read_line(&mut input_two)
        .expect("error");
    
    let input_one:i32 = input_one
        .trim()
        .parse()
        .unwrap();
    let input_two:i32 = input_two
        .trim()
        .parse()
        .unwrap();

    let changer:i32 = input_one / input_two;

    println!("{}", 2022 + changer);
}

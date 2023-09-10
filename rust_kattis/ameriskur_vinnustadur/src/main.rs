use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");

    let football_field:f64 = 0.09144;

    let input: f64 = input
        .trim()
        .parse()
        .unwrap();



    println!("{:?}", input * football_field);
}

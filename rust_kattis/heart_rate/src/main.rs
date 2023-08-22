use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let n: i32 = input.trim()
        .parse()
        .unwrap();

    for _ in 0 .. n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("error");
        let new_input: Vec<f32> = input.trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let min: f32 = new_input[0] - 1.0;
        let max: f32 = new_input[0] + 1.0;
        println!("{:.4} {:.4} {:.4}", ((60.0 * min) / new_input[1]), ((60.0 * new_input[0]) / new_input[1]), ((60.0 * max) / new_input[1]));
    }
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error!");
    let input: i32 = input.trim().parse().unwrap();
    let mut new_input = String::new();
    let mut final_output: f32 = 0.0;
    for i in 0..input{
        new_input.clear();
        io::stdin().read_line(&mut new_input).expect("error!");
        let new_input:Vec<f32> = new_input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        final_output += new_input[0] * new_input[1];
    }
    println!("{:.3}",final_output);
}

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error!");
    let v: Vec<f32> = line.trim().split_whitespace().map(|x| x.parse().unwrap())
    .collect();
    let output:f32 = 0.5*v[0]*v[1];
    println!("{}", output);

}

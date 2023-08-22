use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let n: i32 = input.trim().parse().unwrap();
    let mut output: i32 = 0;

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let (base, rest) = input.trim().split_at(input.len()-2);
        let x: i32 = base.trim().parse().expect("failed at base");
        let y: i32 = rest.trim().parse().expect("failed at rest");
        output += x.pow(y as u32);

    }
    println!("{}", output);
}

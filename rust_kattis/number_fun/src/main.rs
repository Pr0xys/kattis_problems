use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let n: i32 = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let a_b_c: Vec<i32> = input.trim().split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        match a_b_c {
            _ if (a_b_c[0].wrapping_add(a_b_c[1])) ^ a_b_c[2] == 0 ||
            (a_b_c[0].wrapping_sub(a_b_c[1])) ^ a_b_c[2] == 0 ||
            (a_b_c[1].wrapping_sub(a_b_c[0])) ^ a_b_c[2] == 0 ||
            (a_b_c[0].wrapping_mul(a_b_c[1])) ^ a_b_c[2] == 0 ||
            ((a_b_c[0] as f32) / (a_b_c[1] as f32)).to_bits() == (a_b_c[2] as f32).to_bits() ||
            ((a_b_c[1] as f32) / (a_b_c[0] as f32)).to_bits() == (a_b_c[2] as f32).to_bits() => {println!("Possible"); continue},
            _ => {println!("Impossible"); continue},
        }
    }
}
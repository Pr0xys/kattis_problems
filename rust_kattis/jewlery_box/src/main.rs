use std::io;

fn find_max_volume(x: f64, y: f64) -> f64 {
    let discriminant = (x + y).powi(2) - 12.0 * x * y;
    
    if discriminant < 0.0 {
        return 0.0; // Maximum volume can't be found in this case
    }

    let p = (x + y + discriminant.sqrt()) / 12.0;
    let a = x - 2.0 * p;
    let b = y - 2.0 * p;
    let c = p;

    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        return 0.0; // One of the dimensions is non-positive, thus the box can't exist
    }

    a * b * c
}


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please type a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let dimensions: Vec<f64> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (x, y) = (dimensions[0], dimensions[1]);

        let max_volume = find_max_volume(x, y);
        println!("{:.11}", max_volume);
    }
}

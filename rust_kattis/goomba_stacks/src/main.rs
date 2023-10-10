use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = input
        .trim()
        .parse()
        .unwrap();

    let mut total_goombas = 0;
    let mut can_escape = true;  

    for _ in 0..n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let room_data: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let gi = room_data[0];
        let bi = room_data[1]; 

        total_goombas += gi; 

        if total_goombas < bi {
            can_escape = false;
            break;
        }
    }

    if can_escape {
        println!("possible");
    } else {
        println!("impossible");
    }
}

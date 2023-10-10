use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input error");

    let amount_items: i32 = input.trim().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("input error");

    let mut prices: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).expect("input error");

    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("input error");
        let price_changer: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        match price_changer[0].as_str() {
            "INFLATION" => {
                let increment: i64 = price_changer[1].parse().unwrap();
                prices.iter_mut().for_each(|x| *x += increment);

                let output: i64 = prices.iter().sum();
                println!("{}", output);
            }
            "SET" => {
                let target_price: i64 = price_changer[1].parse().unwrap();
                let increment: i64 = price_changer[2].parse().unwrap();
                
                prices.iter_mut().for_each(|x| {
                    if *x == target_price {
                        *x = increment;
                    }
                });

                let output: i64 = prices.iter().sum();
                println!("{}", output);
            }
            _ => println!("lol"),
        }
    }
}


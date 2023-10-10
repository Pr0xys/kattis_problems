use std::collections::HashMap;
use std::io;

fn main() {
    let mut pairs: Vec<(i32, i32)> = Vec::new();

    (0..3).for_each(|_| {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");
        let numbers: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|n| n.parse().expect("Failed to parse"))
            .collect();

        if numbers.len() == 2 {
            pairs.push((numbers[0], numbers[1]));
        }
    });

    let mut first_elements = HashMap::new();
    let mut second_elements = HashMap::new();

    for (first, second) in &pairs {
        *first_elements.entry(first).or_insert(0) += 1;
        *second_elements.entry(second).or_insert(0) += 1;
    }

    let missing_first = find_missing_element(first_elements);
    let missing_second = find_missing_element(second_elements);

    println!(
        "{} {}",
        missing_first, missing_second
    );
}

fn find_missing_element(map: HashMap<&i32, i32>) -> i32 {
    for (key, value) in map.iter() {
        if *value == 1 {
            return **key;
        }
    }
    return 0;
}

use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let contains_smiley = input.contains(":)");
    let contains_frowny = input.contains(":(");

    let output = match (contains_smiley, contains_frowny) {
        (true, false) => "alive",
        (false, true) => "undead",
        (true, true) => "double agent",
        _ => "machine",
    };

    println!("{}", output);
}


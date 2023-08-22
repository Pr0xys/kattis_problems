use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input = input.trim().to_lowercase();
    match input {
        _ if input == "oct 31" => {println!("yup")},
        _ if input == "dec 25" => {println!("yup")},
        _ => {println!("nope")}
    }
}

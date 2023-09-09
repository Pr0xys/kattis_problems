use std::io;
use std::collections::HashMap;

fn newlibrec(x:u64, memo: &mut HashMap<u64,u64>) -> u64 {
    if let Some(&result) = memo.get(&x) {
        return result;
    }

    let result = if x == 1 {
        1
    } else if x == 2 {
        2
    } else {
        let previous = newlibrec(x - 1, memo);
        let before_previous = newlibrec(x - 2, memo);
        previous + before_previous + 1
    };
    
    memo.insert(x, result);
    
    result
}


fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");
    
    let input:u64 = input.trim().parse().unwrap();
    
    let mut memo = HashMap::new();
    
    let output: u64 = newlibrec(input, &mut memo);
    
    println!("{}", output % ((10_u64.pow(9)) + 7_u64));
}

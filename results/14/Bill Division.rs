use std::io::{self, BufRead};

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let total: i32 = bill.iter()
        .enumerate()
        .filter(|&(i, _)| i != k as usize)
        .map(|(_, &x)| x)
        .sum();
    let anna = total / 2;
    if b == anna {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bonAppetit(&bill, k, b);
}

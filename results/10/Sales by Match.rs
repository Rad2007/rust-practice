use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut freq = std::collections::HashMap::new();
    for &s in ar {
        *freq.entry(s).or_insert(0) += 1;
    }
    freq.values().map(|&c| c / 2).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sockMerchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
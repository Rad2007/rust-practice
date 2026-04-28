use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let max_b = *b.iter().max().unwrap();
    let mut count = 0;

    let mut candidate = *a.iter().max().unwrap();
    while candidate <= max_b {
        if a.iter().all(|x| candidate % x == 0)
            && b.iter().all(|x| x % candidate == 0)
        {
            count += 1;
        }
        candidate += 1;
    }

    count
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
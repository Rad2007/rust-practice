use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let (mut max, mut min) = (scores[0], scores[0]);
    let (mut max_count, mut min_count) = (0, 0);

    for &s in &scores[1..] {
        if s > max { max = s; max_count += 1; }
        else if s < min { min = s; min_count += 1; }
    }

    vec![max_count, min_count]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breakingRecords(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

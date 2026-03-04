use std::io::{self, BufRead};
fn staircase(n: i16) {
    for i in 1..=n {
        let speaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", speaces, hashes);
    }
}
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i16>().unwrap();

    staircase(n);
}
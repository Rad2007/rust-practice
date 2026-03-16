use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(grades.len());
    for &grade in grades {
        if grade < 38 {
            result.push(grade);
        }
        else {
            let remainder = grade % 5;
            let next_multiple = if remainder == 0 {
                grade
            }
            else {
                grade + (5 - remainder)
            };
            if next_multiple - grade < 3 {
                result.push(next_multiple);
            }
            else {
                result.push(grade);
            }
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = grading_students(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }
    writeln!(&mut fptr).ok();
}
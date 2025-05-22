use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn int_from_file(filename: impl AsRef<Path>) -> Vec<i64> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    let numbers: Vec<i64> = buf
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();
    numbers
}

fn main() {
    let input = int_from_file("data/puzzle.txt");

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            for k in i + 1..input.len() {
                let a = input[i];
                let b = input[j];
                let c = input[k];

                let sum = a + b + c;
                if sum == 2020 {
                    println!("{a} + {b} + {c} = {}", sum);
                    println!("donc {a} x {b} x {c} = {}", a * b * c);
                }
            }
        }
    }
}

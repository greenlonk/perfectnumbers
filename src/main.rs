use rayon::prelude::*;
use std::process::exit;
use std::time::Instant;
use std::{env, io};

fn main() {
    let n: usize = parse_args();
    let start = Instant::now();
    let test = get_all_perfect_numbers(n).expect("No perfect numbers found!");
    let stop = Instant::now();
    let time = stop.duration_since(start).as_millis();

    println!(
        "\nPerfect numbers up to {}: \n{:?} \n\
        Calculation took {} ms \n\
        Average time per step: {} µs",
        n,
        test,
        time,
        stop.duration_since(start).as_micros() / n as u128
    );
}

fn parse_args() -> usize {
    // Parse args
    let args: Vec<String> = env::args().collect();

    let mut samplecount: usize = 0;

    // Parse samplecount from CLI args or ask for it
    if args.len() >= 2 {
        samplecount = args[1].parse().expect("Please input a number!");
    }

    if samplecount <= 0 {
        println!(
            "Maximum number was not given or is less than one. How many samples would you like to use? "
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        samplecount = input.trim().parse().expect("Please input a non-zero, positive number!");
    }

    if samplecount == 0 {
        println!("Samplecount cannot be less than one.");
        exit(1);
    }
    samplecount
}

fn check_even_dividers(n: usize) -> Vec<usize> {
    (1..=(n / 2)).filter(|i| n % i == 0).collect()
}

fn check_uneven_dividers(n: usize) -> Vec<usize> {
    (1..=(n / 3)).step_by(2).filter(|i| n % i == 0).collect()
}

fn check_perfect_number(n: usize) -> bool {
    let if_even = n % 2 == 0;
    let divider_vec: Vec<usize> = if if_even {
        check_even_dividers(n)
    } else {
        check_uneven_dividers(n)
    };
    let sum: usize = divider_vec.iter().sum();
    sum == n
}

fn get_all_perfect_numbers(n: usize) -> Option<Vec<usize>> {
    let perfect_vec: Vec<usize> = (1..=n)
        .into_par_iter()
        .filter(|&i| check_perfect_number(i))
        .collect();
    if perfect_vec.is_empty() {
        None
    } else {
        Some(perfect_vec)
    }
}

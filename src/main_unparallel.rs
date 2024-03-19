// Version that doesn't utilize multiprocessing with Rayon

use std::process::exit;
use std::time::Instant;
use std::{env};
use std::io::Write;

fn main() {
    let n: usize = parse_args();
    let start = Instant::now();
    let test = get_all_perfect_numbers(n).expect("No perfect numbers found!");
    let stop = Instant::now();
    let time = stop.duration_since(start).as_millis();

    println!("\nPerfect numbers up to {n}: \n{:?} \n\
        Calculation took {} ms \n\
        Average time per step: {} µs", test, time, stop.duration_since(start).as_micros() / n as u128)
}

fn parse_args() -> usize{
    // Parse args
    let args: Vec<String> = env::args().collect();

    let mut samplecount: usize = 0;

    // Parse samplecount from CLI args or ask for it
    if args.len() >= 2 {
        samplecount = args[1].parse().expect("Please input a number!");
    }

    if samplecount <= 0 {
        println!("Maximum number was not given or is less than one. How many samples would you like to use? ");
        let mut input = String::new();
        std::io::stdin()
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

fn check_even_dividers(n: usize) -> Vec<usize>{
    let mut divider_vec: Vec<usize> = vec![];
    for i in 1..(n/2 + 1){
        if n % i == 0{
            divider_vec.push(i);
        } else {
        }
    }
    divider_vec
}

fn check_uneven_dividers(n: usize) -> Vec<usize>{
    let mut divider_vec: Vec<usize> = vec![];
    for i in (1..(n/3 + 1)).step_by(2){
        if n % i == 0{
            divider_vec.push(i);
        } else {
        }
    }
    divider_vec
}

fn check_perfect_number(n: usize) -> bool{
    let if_even = n % 2 == 0;
    let divider_vec: Vec<usize> = match if_even{
        true => check_even_dividers(n),
        false => check_uneven_dividers(n),
    };
    let sum: usize = divider_vec.iter().sum();
    if sum == n{
        true
    } else {
        false
    }
}

fn get_all_perfect_numbers(n: usize) -> Option<Vec<usize>>{
    let mut perfect_vec: Vec<usize> = vec![];
    for i in 1..n+1{
        let sub_start: Instant = Instant::now();
        if check_perfect_number(i){
            perfect_vec.append(&mut vec![i]);
        }
        if i%1000==0{
            let sub_stop: Instant = Instant::now();
            print!("\rSubstep time: {} µs \
                                ", sub_stop.duration_since(sub_start).as_micros());
            std::io::stdout().flush();
        }
    }
    if perfect_vec == vec![]{
        None
    } else {
        Some(perfect_vec)
    }

}

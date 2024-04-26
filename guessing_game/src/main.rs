use core::time;
use rand::{self, rngs, Rng};
use std::{io, thread::sleep};

fn type_number() -> i32 {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fail to read!");
    let guess: i32 = guess.trim().parse().expect("Please type a number!");
    return guess;
}

fn main() {
    println!("Let's play guessing game!");
    println!("Input for number: ");

    const MAX: i32 = 10000000;
    let mut guess = MAX / 2;
    let mut gap = MAX / 4;
    let mut count = 1;

    println!("Your number: {guess}");
    let value = rand::thread_rng().gen_range(0..=MAX);

    while value != guess {
        match guess.cmp(&value) {
            std::cmp::Ordering::Less => {
                guess = guess + gap
            }
            std::cmp::Ordering::Equal => {
                break;
            }
            std::cmp::Ordering::Greater => {
                guess = guess - gap
            }
        }
        println!("gap: {gap}");
        gap = gap / 2;
        if gap == 0 {
            gap = 1;
        }
        // sleep(time::Duration::from_millis(2000));
        count += 1;
    }
    println!("You Win!");
    println!("Guess count: {count}");
}

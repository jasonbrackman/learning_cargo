extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

fn main() {

    guess_number();

    let number = -50.0;
    println!("{} Degrees C is in F: {}", number, convert_celcius_to_fahrenheit(number));


    let mut hmap: HashMap<u128, u128> = HashMap::new();

    // let x = 10;
    for x in 0..100 {
        println!("RESULT [{}]: {}", x, fib(x, &mut hmap))
    }

}


fn fib(n: u128, map: &mut HashMap<u128, u128>) -> u128 {
    // hoping to early exit here...
    if let Some(value) = map.get(&n) {
        //println!("Should be cache hit: {:?}", value);
        return *value
    }

    if n < 2 {
        map.insert(n, n);
        n
    } else {
        let total = fib(n-2, map) + fib(n-1, map);
        map.insert(n, total);
        total
    }
}

fn guess_number() -> () {
    println!("Guess the number:");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        println!("Please enter your guess:");

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low"),
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn convert_celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 1.8 + 32.0
}


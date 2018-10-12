extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;
mod fib;
use fib::fib;

fn main() {


    // These items below cover chapters 1 - 4
    twelve_days_of_christmas();

    guess_number();

    let number = -50.0;
    println!("{} Degrees C is in F: {}", number, convert_celcius_to_fahrenheit(number));


    // let hmap: HashMap<u64, u64> = HashMap::new();
    let mut hmap: HashMap<u64, u64> = HashMap::new();

    for x in 0..84 {
        println!("RESULT [{}]: {}", x, fib(x, &mut hmap));
    }
}


fn guess_number() -> () {
    println!("Guess the number:");
    let secret_number = rand::thread_rng().gen_range(1, 2);
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

fn twelve_days_of_christmas() {
    /*
    On the first day of Christmas, my true love sent to me
    A partridge in a pear tree.

    ...

    On the twelfth day of Christmas, my true love gave to me
    Twelve drummers drumming, eleven pipers piping, ten lords a-leaping, nine ladies dancing,
    eight maids a-milking, seven swans a-swimming, six geese a-laying,
    five golden rings, four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.
    */
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
                          "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["Twelve drummers drumming", "eleven pipers piping", "ten lords a-leaping",
                           "nine ladies dancing", "eight maids a-milking", "seven swans a-swimming",
                           "six geese a-laying", "Five golden rings", "Four calling birds",
                           "three French hens", "two turtle doves", "a partridge in a pear tree"];

    for (counter, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love game to me", day);

        let max = gifts.len() - 1;
        for gift in gifts[(max - counter..max)].iter() {
            println!("\t{}", gift);
        }

        let prefix = if counter != 0 {"And "} else {""};
        println!("\t{}{}.\n", prefix, gifts[max]);
    }

}
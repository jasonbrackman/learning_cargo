extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    twelve_days_of_christmas();

    guess_number();

    let number = -50.0;
    println!("{} Degrees C is in F: {}", number, convert_celcius_to_fahrenheit(number));


    // let hmap: HashMap<u64, u64> = HashMap::new();
    let mut hmap: HashMap<u64, u64> = [(0, 0), (1, 1)].into_iter().cloned().collect();

    for x in 0..80 {
        println!("RESULT [{}]: {}", x, fib(x, &mut hmap))
    }
}


fn fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    // hoping to early exit here...
    if let Some(value) = map.get(&n) {
        return *value
    }

    let total = fib(n - 2, map) + fib(n - 1, map);
    map.insert(n, total);
    return total

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

    On the second day of Christmas, my true love sent to me
    Two turtle doves
    and a partridge in a pear tree.

    On the third day of Christmas, my true love sent to me
    Three French hens, two turtle doves
    And a partridge in a pear tree.

    On the fourth day of Christmas, my true love sent to me
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.

    On the fifth day of Christmas, my true love sent to me
    Five golden rings.
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.

    On the sixth day of Christmas, my true love gave to me
    Six geese a-laying,
    Five golden rings.
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.

    On the seventh day of Christmas, my true love gave to me
    Seven swans a-swimming, six geese a-laying,
    Five golden rings.
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.

    On the eighth day of Christmas, my true love gave to me
    Eight maids a-milking, seven swans a-swimming, six geese a-laying,
    Five golden rings.
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.

    On the ninth day of Christmas, my true love gave to me
    Nine ladies dancing, eight maids a-milking, seven swans a-swimming, six geese a-laying,
    Five golden rings.
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.

    On the tenth day of Christmas, my true love gave to me
    Ten lords a-leaping, nine ladies dancing, eight maids a-milking, seven swans a-swi'mmi'ng, six geese a-laying,
    Five golden rings.
    Fou'r calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.

    On the eleventh day of Christmas, my true love gave to me
    Eleven pipers piping, ten lords a-leaping, nine ladies dancing, eight maids a-milking, seven swans a-swimming,
    Six geese a-laying,
    Five golden rings.
    Four calling birds, three French hens, two turtle doves And a partridge in a pear tree.

    On the twelfth day of Christmas, my true love gave to me
    Twelve drummers drumming, eleven pipers piping, ten lords a-leaping, nine ladies dancing, eight maids a-milking, seven swans a-swimming, six geese a-laying,
    Five golden rings.
    Four calling birds, three French hens, two turtle doves
    And a partridge in a pear tree.
    */
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
                          "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["Twelve drummers drumming", "eleven pipers piping", "ten lords a-leaping",
                           "nine ladies dancing", "eight maids a-milking", "seven swans a-swimming",
                           "six geese a-laying", "Five golden rings", "Four calling birds",
                           "three French hens", "two turtle doves", "a partridge in a pear tree"];

    let mut counter = 1;
    for day in days.iter() {
        println!("On the {} day of Christmas, my true love game to me", day);

        for gift in gifts[(gifts.len() - counter..gifts.len() - 1)].iter() {
            println!("\t{}", gift);
        }

        if counter == 1 {
            println!("\t{}.", gifts[gifts.len() - 1]);
        }
        else {
            println!("\tAnd {}.", gifts[gifts.len() - 1]);
        }
        println!("\n");

        counter += 1;
    }

}
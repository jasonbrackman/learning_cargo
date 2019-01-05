extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

mod fib;


fn quicksort(mut items: Vec<i32>) -> Vec<i32> {
    // base case
    if items.len() < 2 {
        return items;
    }

    // recursive case
    let num = rand::thread_rng().gen_range(0, items.len());
    let pivot = items.remove(num);
    let (less,more) = items.into_iter().partition(|&x| x < pivot);

    let mut temp = Vec::new();
    temp.extend(quicksort(less));
    temp.extend(vec![pivot]);
    temp.extend(quicksort(more));
    return temp;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn main() {
    let items = vec![10, 7, 15, 70, 50, 500, 100, 0, 6, 2, 11, 3, 4, 40, 5];
    println!("{:?}", quicksort(items));
    // fib::fib(10);
    let _player_direction: Direction = Direction::Up;

    // These items below cover chapters 1 - 4
    twelve_days_of_christmas();

    guess_number();

    let number = -50.0;
    println!(
        "{} Degrees C is in F: {}",
        number,
        convert_celcius_to_fahrenheit(number)
    );

    let mut hmap: HashMap<u64, u64> = HashMap::new();

    for x in 0..184 {
        println!("RESULT [{}]: {}", x, fib::fib(x, &mut hmap));
    }

    // From Chapter 5
    let rect = Rectangle {
        width: 50,
        height: 30,
    };
    println!("Area of a the {:?} = {}", rect, rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn guess_number() {
    println!("Guess the number:");
    let secret_number = rand::thread_rng().gen_range(1, 2);
    loop {
        let mut guess = String::new();
        println!("Please enter your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: i32 = match guess.trim().parse() {
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
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "Twelve drummers drumming",
        "eleven pipers piping",
        "ten lords a-leaping",
        "nine ladies dancing",
        "eight maids a-milking",
        "seven swans a-swimming",
        "six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "three French hens",
        "two turtle doves",
        "a partridge in a pear tree",
    ];

    for (counter, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, my true love game to me", day);

        let max = gifts.len() - 1;
        for gift in gifts[(max - counter..max)].iter() {
            println!("\t{}", gift);
        }

        let prefix = if counter != 0 { "And " } else { "" };
        println!("\t{}{}.\n", prefix, gifts[max]);
    }
}

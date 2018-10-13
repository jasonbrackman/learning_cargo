
use std::collections::HashMap;

pub fn fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {

    match n {
        0 => 0,
        1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let total = fib(n - 2, map) + fib(n - 1, map);
                map.insert(n, total);
                total
            }
        }
    }
}
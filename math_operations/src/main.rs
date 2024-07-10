// src/main.rs

use math_operations::*;

fn main() {
    let a = 10;
    let b = 5;

    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, subtract(a, b));
    println!("{} * {} = {}", a, b, multiply(a, b));
    match divide(a, b) {
        Some(result) => println!("{} / {} = {}", a, b, result),
        None => println!("Cannot divide by zero"),
    }
}

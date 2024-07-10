// src/main.rs

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

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-2, -3), -5);
        assert_eq!(add(2, -3), -1);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(-5, -3), -2);
        assert_eq!(subtract(5, -3), 8);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(-2, -3), 6);
        assert_eq!(multiply(2, -3), -6);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6, 3), Some(2));
        assert_eq!(divide(6, 0), None);
        assert_eq!(divide(-6, -3), Some(2));
        assert_eq!(divide(6, -3), Some(-2));
    }
}

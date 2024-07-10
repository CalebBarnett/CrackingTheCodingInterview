// tests/math_operations.rs

use math_operations::*;
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
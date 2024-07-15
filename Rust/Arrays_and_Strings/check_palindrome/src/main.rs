fn is_palindrome(string: &str) -> bool {
    //create new string called new_string from passed in string after
    //filtering out non alphanumeric characters and converting to lowercase
    let new_string: String = string.chars()
        .filter(|character| character.is_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect();

    new_string == new_string.chars().rev().collect::<String>()
}

fn main() {
    let string = "taco cat";
    println!("'{}' is a palindrome: {}", string, is_palindrome(string));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_test() {
        assert_eq!(is_palindrome("taco cat"), true);
        assert_eq!(is_palindrome("dog"), false);
    }
}
fn urlify(s: &str) -> String {
    let trimmed_str = s.trim_end();
    trimmed_str.replace(" ", "%20")
}

fn main() {
    let test_str = "Mr John Smith   ";
    println!("'{}' URLify'd is {}", test_str, urlify(test_str));
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    // Test case function
    #[test]
    fn test_urlify() {
        // Input string
        let test_str = "Mr John Smith   ";
        
        // Expected output after URLify
        let expected_output = "Mr%20John%20Smith";

        // Call the function and check the result
        let result = urlify(test_str);
        assert_eq!(result, expected_output);
    }
}
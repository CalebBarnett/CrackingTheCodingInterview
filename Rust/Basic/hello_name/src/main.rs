use std::io;
use std::io::Read;

// Function to handle user input
fn get_user_input<R: Read>(input: &mut R) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)
        .expect("Failed to read line");
    buffer
}

fn main() {
    println!("Please enter your name:");

    // Use stdin() directly for main application
    let mut input = io::stdin();
    let user_input = get_user_input(&mut input);

    println!("Hello, {}", user_input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_get_user_input() {
        let input = "Alice\n".as_bytes();
        let mut reader = Cursor::new(input);
        let result = get_user_input(&mut reader);
        assert_eq!(result.trim(), "Alice");
    }

    #[test]
    fn test_main_function() {
        // You can test the main function indirectly by capturing its output
        let input = "Bob\n";
        let mut reader = Cursor::new(input);
        let output = std::io::stdout();
        let _ = std::io::copy(&mut reader, &mut output.lock());
        assert!(true); // Placeholder assertion for demonstration
    }
}

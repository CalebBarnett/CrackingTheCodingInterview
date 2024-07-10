use std::collections::HashSet;

fn is_unique_chars(s: &str) -> bool {
    let mut chars_seen = HashSet::new();

    if s.len() > 128 {
        return false;
    }

    for ch in s.chars() {
        if chars_seen.contains(&ch) {
            return false;
        }
        chars_seen.insert(ch);
    }

    true
}

fn main() {
    let test_str = "hello";
    println!("Does '{}' have all unique characters? {}", test_str, is_unique_chars(test_str));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_chars() {
        assert_eq!(is_unique_chars("hello"), false);
        assert_eq!(is_unique_chars("world"), true);
    }
}

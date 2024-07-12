
//sort string
fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect(); 
    chars.sort(); 
    chars.into_iter().collect()
}

//check permutation
fn check_permutation(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false
    }
    sort_string(a) == sort_string(b)
}

fn main() {
    let test_str_one = "dog";
    let test_str_two = "god";
    println!("'{}' and '{}' are permutations: {}", test_str_one, test_str_two, check_permutation(test_str_one, test_str_two));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_test() {
        assert_eq!(check_permutation("hello", "world"), false);
        assert_eq!(check_permutation("dog", "god"), true);
    }
}
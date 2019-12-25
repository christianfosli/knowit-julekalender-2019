fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

fn is_hidden_palindrome(n: u64) -> bool {
    if is_palindrome(&format!("{}", n)) {
        return false;
    }
    let n_plus_rev = &format!(
        "{}",
        n + format!("{}", n)
            .chars()
            .rev()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    );

    is_palindrome(&format!("{}", n_plus_rev))
}

fn main() {
    let mut sum = 0;
    for i in 1..123_454_321 {
        if is_hidden_palindrome(i) {
            sum += i;
        }
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hidden_palindrome_should_be_true() {
        assert_eq!(is_hidden_palindrome(38), true);
    }

    #[test]
    fn test_is_hidden_palindrome_should_be_false() {
        assert_eq!(is_hidden_palindrome(49), false);
    }
}

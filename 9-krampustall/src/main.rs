use std::fs;

fn main() {
    let nums: Vec<u64> = fs::read_to_string("krampus.txt")
        .expect("Error reading file")
        .split("\n")
        .map(|s| s.parse().expect("Unable to parse to num"))
        .collect();
    let mut sum = 0;
    for n in nums {
        if is_krampus(n) {
            sum += n;
        }
    }
    println!("{}", sum);
}

fn is_krampus(n: u64) -> bool {
    let squared: u64 = n.pow(2);
    let squared_str: String = format!("{}", squared);
    for i in 1..squared_str.len() {
        let (n1, n2) = squared_str.split_at(i);
        let n1: u64 = n1.parse().expect("Unable to parse char back to num");
        let n2: u64 = n2.parse().expect("Unable to parse char back to num");
        if n1 == 0 || n2 == 0 {
            continue;
        }
        if n1 + n2 == n {
            return true;
        }
    }
    false
}

#[test]
fn test_is_krampus_given_krampustall_should_be_true() {
    let n = 45;
    assert_eq!(is_krampus(n), true);
}

#[test]
fn test_is_krampus_given_not_krampustall_should_be_false() {
    let n = 100;
    assert_eq!(is_krampus(n), false);
}

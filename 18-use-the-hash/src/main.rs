use std::collections::HashMap;
use std::fs;

enum Sex {
    Male,
    Female,
}

fn use_the_hash(fname: &str, lname: &str, sex: Sex) -> String {
    let names = fs::read_to_string("names.txt").unwrap();
    let names: Vec<&str> = names.split("---").collect();
    let fnames: Vec<&str> = match sex {
        Sex::Male => names[0].trim().lines().collect(),
        Sex::Female => names[1].trim().lines().collect(),
    };
    let lnames_part_1: Vec<&str> = names[2].trim().lines().collect();
    let lnames_part_2: Vec<&str> = names[3].trim().lines().collect();

    let new_fname = fnames[sum_as_ascii(&fname) % fnames.len()];

    let (lname_1, lname_2) = lname.split_at((lname.len() as f64 / 2.0).ceil() as usize);
    let lname_part_1 = lnames_part_1[sum_as_alphabet_values(&lname_1) % lnames_part_1.len()];
    let lname_part_2 = match sex {
        Sex::Male => {
            lnames_part_2[sort_desc(product_as_ascii(&lname_2) * fname.len()) % lnames_part_2.len()]
        }
        Sex::Female => {
            lnames_part_2[sort_desc(product_as_ascii(&lname_2) * (fname.len() + lname.len()))
                % lnames_part_2.len()]
        }
    };

    format!("{} {}{}", new_fname, lname_part_1, lname_part_2)
}

fn sum_as_ascii(s: &str) -> usize {
    let mut sum = 0;
    for c in s.chars() {
        sum += c as usize;
    }
    sum
}

fn product_as_ascii(s: &str) -> usize {
    let mut prod = 1;
    for c in s.chars() {
        prod *= c as usize;
    }
    prod
}

fn sum_as_alphabet_values(s: &str) -> usize {
    let s = s.to_ascii_lowercase();
    let mut sum = 0;
    for c in s.chars() {
        sum += c as usize % ('a' as usize - 1)
    }
    sum
}

fn sort_desc(n: usize) -> usize {
    let mut chars: Vec<char> = format!("{}", n).chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    chars
        .into_iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn main() {
    let employees = fs::read_to_string("employees.csv").unwrap();
    let mut names = HashMap::new();
    for l in employees.lines() {
        let split: Vec<_> = l.split(",").collect();
        let (fname, lname, gender) = (
            split[0],
            split[1],
            match split[2] {
                "M" => Sex::Male,
                "F" => Sex::Female,
                _ => panic!("Unexpected gender"),
            },
        );
        let hash = use_the_hash(fname, lname, gender);
        let name_cnt = names.entry(hash).or_insert(0);
        *name_cnt += 1;
    }
    let mut names: Vec<_> = names.drain().collect();
    names.sort_by(|a, b| b.1.cmp(&a.1));
    let (winner, count) = names.first().unwrap();
    println!("{} {}", winner, count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_the_hash() {
        assert_eq!(
            use_the_hash("Jan", "Johannsen", Sex::Male),
            "Poe Lightverse"
        );
    }

    #[test]
    fn test_sum_as_ascii() {
        assert_eq!(sum_as_ascii("Jan"), 281);
    }

    #[test]
    fn test_sum_as_alphabet_values() {
        assert_eq!(sum_as_alphabet_values("Johan"), 48);
    }

    #[test]
    fn test_product_as_ascii() {
        assert_eq!(product_as_ascii("nsen"), 140541500);
    }

    #[test]
    fn test_sort_desc() {
        assert_eq!(sort_desc(123), 321);
    }
}

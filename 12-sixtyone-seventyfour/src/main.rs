fn main() {
    let mut seven_count = 0;
    for i in 1000..10_000 {
        if i % 1111 == 0 {
            continue;
        }
        let iterations = iterations_til_6174(i);
        if iterations == 7 {
            seven_count += 1;
        }
    }
    println!("{}", seven_count);
}

fn iterations_til_6174(n: u32) -> u32 {
    let mut iterations = 0;
    let mut n = n;
    while n != 6174 {
        if iterations > 7 {
            panic!("too many iterations!! for n: {}", n);
        }
        n = sort_and_subtract(n);
        iterations += 1;
    }
    iterations
}

fn sort_and_subtract(n: u32) -> u32 {
    let mut chars: Vec<char> = format!("{:04}", n).chars().collect();
    chars.sort();
    let asc: u32 = chars.iter().collect::<String>().parse().unwrap();
    chars.reverse();
    let desc: u32 = chars.iter().collect::<String>().parse().unwrap();
    desc - asc
}

#[test]
fn test_iterations_til_6147() {
    assert_eq!(iterations_til_6174(1000), 5);
}

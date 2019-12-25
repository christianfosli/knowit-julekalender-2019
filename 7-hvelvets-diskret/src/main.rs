use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!(
        "code: {}. Time used: {}",
        special_division(7),
        start.elapsed().as_millis()
    );
}

fn special_division(x: i64) -> i64 {
    let mut y: Option<i64> = None;
    for y_prime in 2..27644437 {
        let b: i64 = y_prime * x;
        let r: i64 = b % 27644437;
        if r == 1 {
            y = Some(y_prime);
            break;
        }
    }
    match y {
        Some(y) => {
            let z: i64 = 5897 * y;
            return z % 27644437;
        }
        None => panic!("No y found!"),
    }
}

#[test]
fn test_special_division_day_2() {
    assert_eq!(special_division(2), 13825167);
}

use primes::is_prime;
use std::sync::mpsc;
use std::thread;

fn is_harshad_prime(n: u32) -> bool {
    let sum_digits = sum_digits(n);
    let is_harshad = n % sum_digits == 0;
    is_harshad && is_prime(sum_digits as u64)
}

fn sum_digits(n: u32) -> u32 {
    let mut n = n;
    let mut sum = 0;
    for _ in 0..((n as f32).log10() as u32 + 1) {
        sum += n % 10;
        n = n / 10;
    }
    sum
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx);
    let tx3 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let mut count = 0;
        for i in 1..(98_765_433 / 4) {
            if is_harshad_prime(i) {
                count += 1;
            }
        }
        tx.send(count).unwrap();
    });
    thread::spawn(move || {
        let mut count = 0;
        for i in (98_765_433 / 4)..(98_765_433 / 4 * 2) {
            if is_harshad_prime(i) {
                count += 1;
            }
        }
        tx1.send(count).unwrap();
    });
    thread::spawn(move || {
        let mut count = 0;
        for i in (98_765_433 / 4 * 2)..(98_765_433 / 4 * 3) {
            if is_harshad_prime(i) {
                count += 1;
            }
        }
        tx2.send(count).unwrap();
    });
    thread::spawn(move || {
        let mut count = 0;
        for i in (98_765_433 / 4 * 3)..(98_765_433) {
            if is_harshad_prime(i) {
                count += 1;
            }
        }
        tx3.send(count).unwrap();
    });
    println!(
        "{}",
        rx.recv().unwrap() + rx.recv().unwrap() + rx.recv().unwrap() + rx.recv().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_digs() {
        assert_eq!(sum_digits(123), 6);
    }

    #[test]
    fn test_is_harshad_prime_should_be_true() {
        assert_eq!(is_harshad_prime(1729), true);
    }

    #[test]
    fn test_is_harshad_prime_should_be_false() {
        assert_eq!(is_harshad_prime(1730), false);
    }
}

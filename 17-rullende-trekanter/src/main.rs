fn main() {
    let mut triangular_and_square_count = 0;
    for i in 0..1_000_000 {
        let an = triangular_num(i);
        if an.is_square() {
            triangular_and_square_count += 1;
            continue;
        }
        if an.roll().iter().any(|r| r.is_square()) {
            triangular_and_square_count += 1;
        }
    }
    println!("{}", triangular_and_square_count);
}

pub trait U64Extensions {
    fn roll(&self) -> Vec<u64>;
    fn is_square(&self) -> bool;
}

impl U64Extensions for u64 {
    fn roll(&self) -> Vec<u64> {
        let mut rolled: Vec<u64> = Vec::new();
        let mut pow = 10;
        while pow <= *self {
            let multiplier = (10 as f64)
                .powi((*self as f64).log10() as i32 - (pow as f64 / 10.0f64).log10() as i32)
                as u64;
            let last_digit = self % pow;
            let the_others = self / pow;
            let r = last_digit * multiplier + the_others;
            rolled.push(r);
            pow *= 10;
        }
        rolled
    }

    fn is_square(&self) -> bool {
        let sqrt = (*self as f64).sqrt();
        sqrt.fract() == 0.0
    }
}

fn triangular_num(n: u64) -> u64 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_roll() {
        assert_eq!(1230.roll(), vec![0123, 3012, 2301]);
    }

    #[test]
    fn test_triangular_num() {
        assert_eq!(triangular_num(0), 0);
        assert_eq!(triangular_num(1), 1);
        assert_eq!(triangular_num(10), 55);
    }

    #[test]
    fn test_triangular_num_doesnt_overflow() {
        triangular_num(1_000_000);
    }

    #[test]
    fn test_is_square() {
        assert_eq!(36.is_square(), true);
        assert_eq!(35.is_square(), false);
    }
}

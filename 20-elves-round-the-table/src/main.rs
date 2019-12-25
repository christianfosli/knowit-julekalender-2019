use primes;
use std::cmp::Ordering;
use std::u32;

#[derive(Default)]
struct Elf {
    workCount: u32,
}

enum Dir {
    Clockwise,
    Counterclockwise,
}

///
/// Modulo that handles negative numbers, works the same as Python's `%`.
///
/// eg: `(a + b).modulo(c)`
///
/// Ref StackOverflow: https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
///
pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}
macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt for $t {
            #[inline]
            fn modulo(&self, n: Self) -> Self {
                (self % n + n) % n
            }
        }
    )*)
}
modulo_signed_ext_impl! { i8 i16 i32 i64 }

fn build_present() {
    let mut elves = [
        Elf {
            ..Default::default()
        },
        Elf {
            ..Default::default()
        },
        Elf {
            ..Default::default()
        },
        Elf {
            ..Default::default()
        },
        Elf {
            ..Default::default()
        },
    ];
    let mut dir = Dir::Clockwise;
    let mut current_elf: i64 = 0;
    let mut stage = 1;
    elves[0].workCount += 1;
    while stage < 1_000_740 {
        stage += 1;
        if primes::is_prime(stage as u64) {
            let res = least_work(&elves);
            if res.is_ok() {
                current_elf = res.unwrap() as i64;
                elves[current_elf as usize].workCount += 1;
                continue;
            }
        }
        if stage % 28 == 0 {
            dir = match dir {
                Dir::Clockwise => Dir::Counterclockwise,
                Dir::Counterclockwise => Dir::Clockwise,
            };
            current_elf = match dir {
                Dir::Clockwise => (current_elf + 1).modulo(5),
                Dir::Counterclockwise => (current_elf - 1).modulo(5),
            };
            elves[current_elf as usize].workCount += 1;
            continue;
        }
        if stage % 2 == 0
            && elves[match dir {
                Dir::Clockwise => (current_elf + 1).modulo(5),
                Dir::Counterclockwise => (current_elf - 1).modulo(5),
            } as usize]
                .workCount
                == most_work(&elves).0
            && most_work(&elves).1
        {
            current_elf = match dir {
                Dir::Clockwise => (current_elf + 2).modulo(5),
                Dir::Counterclockwise => (current_elf - 2).modulo(5),
            };
            elves[current_elf as usize].workCount += 1;
            continue;
        }
        if stage % 7 == 0 {
            current_elf = 4;
            elves[current_elf as usize].workCount += 1;
            continue;
        }
        current_elf = match dir {
            Dir::Clockwise => (current_elf + 1).modulo(5),
            Dir::Counterclockwise => ((current_elf - 1).modulo(5)),
        };
        elves[current_elf as usize].workCount += 1;
    }
    println!("{}", most_work(&elves).0 - least_work_num(&elves));
}

// The index of the elf that has done the least work
fn least_work(elves: &[Elf; 5]) -> Result<usize, &str> {
    const INVALID: u32 = u32::MAX;
    let mut min = INVALID;
    let mut idx = INVALID;
    for (i, e) in elves.iter().enumerate() {
        match e.workCount.cmp(&min) {
            Ordering::Less => {
                min = e.workCount;
                idx = i as u32;
            }
            Ordering::Equal => idx = INVALID,
            Ordering::Greater => {}
        }
    }
    if idx == INVALID {
        return Err("more than one elf matches");
    }
    Ok(idx as usize)
}

fn least_work_num(elves: &[Elf; 5]) -> u32 {
    let mut min = u32::MAX;
    for e in elves {
        if e.workCount < min {
            min = e.workCount;
        }
    }
    min
}

// The amount of work done at most
fn most_work(elves: &[Elf; 5]) -> (u32, bool) {
    let mut max = 0;
    let mut unique = false;
    for e in elves {
        if e.workCount > max {
            max = e.workCount;
            unique = true;
        } else if e.workCount == max {
            unique = false;
        }
    }
    (max, unique)
}

fn main() {
    build_present();
}

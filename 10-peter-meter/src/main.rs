extern crate chrono;

use chrono::{Datelike, NaiveDate, Weekday};
use regex::Regex;
use std::fs;

struct Usage {
    toothpaste_ml: u64,
    shampoo_ml: u64,
    shampoo_ml_sundays_only: u64,
    toilet_paper_m: u64,
    toilet_paper_m_wednesdays_only: u64,
}

impl Usage {
    fn tubes_of_toothpaste(&self) -> u64 {
        self.toothpaste_ml / 125
    }
    fn tubes_of_shampoo(&self) -> u64 {
        self.shampoo_ml / 300
    }
    fn rolls_of_tp(&self) -> u64 {
        self.toilet_paper_m / 25
    }
    fn secret_code(&self) -> u64 {
        self.tubes_of_toothpaste()
            * self.tubes_of_shampoo()
            * self.rolls_of_tp()
            * self.shampoo_ml_sundays_only
            * self.toilet_paper_m_wednesdays_only
    }
}

fn main() {
    let usage = parse_usage_from_file();
    println!("{}", usage.secret_code());
}

fn parse_usage_from_file() -> Usage {
    let mut usage = Usage {
        toothpaste_ml: 0,
        shampoo_ml: 0,
        shampoo_ml_sundays_only: 0,
        toilet_paper_m: 0,
        toilet_paper_m_wednesdays_only: 0,
    };
    let date_re = Regex::new(r"(?P<month>^\S\S\S) (?P<day>\d\d)").unwrap();
    let toothpaste_re = Regex::new(r"(\d+) ml tannkrem").unwrap();
    let shampoo_re = Regex::new(r"(\d+) ml sjampo").unwrap();
    let tp_re = Regex::new(r"(\d+) meter toalettpapir").unwrap();

    let log = fs::read_to_string("logg.txt").expect("Error reading file");
    let mut date: NaiveDate =
        NaiveDate::parse_from_str("01 Jan 1970", "%d %b %Y").expect("Unable to parse date");
    for l in log.lines() {
        if date_re.is_match(&l) {
            let captures = date_re.captures(l).unwrap();
            date = NaiveDate::parse_from_str(
                &format!("{} {} 2018", &captures["day"], &captures["month"]),
                "%d %b %Y",
            )
            .expect("Unable to parse date");
        } else if toothpaste_re.is_match(&l) {
            let captures = toothpaste_re.captures(l).unwrap();
            let toothpaste: u64 = captures
                .get(1)
                .expect("Unable to get toothepaste")
                .as_str()
                .parse()
                .expect("Unable to parse toothpaste to num");
            usage.toothpaste_ml += toothpaste;
        } else if shampoo_re.is_match(&l) {
            let captures = shampoo_re.captures(&l).unwrap();
            let shampoo: u64 = captures
                .get(1)
                .expect("Unable to get shampoo")
                .as_str()
                .parse()
                .expect("Unable to parse shampoo to num");
            usage.shampoo_ml += shampoo;
            if date.weekday() == Weekday::Sun {
                usage.shampoo_ml_sundays_only += shampoo;
            }
        } else if tp_re.is_match(&l) {
            let captures = tp_re.captures(&l).unwrap();
            let tp: u64 = captures
                .get(1)
                .expect("Unable to get TP")
                .as_str()
                .parse()
                .expect("Unable to parse TP to num");
            usage.toilet_paper_m += tp;
            if date.weekday() == Weekday::Wed {
                usage.toilet_paper_m_wednesdays_only += tp;
            }
        } else {
            println!("Woops -- I don't know what `{}` is supposed to mean", &l);
        }
    }
    usage
}

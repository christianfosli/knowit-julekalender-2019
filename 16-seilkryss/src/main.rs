use std::fs;

struct Birte {
    x: usize,
    y: usize,
    dir: &'static Direction,
}

#[derive(PartialEq)]
enum Direction {
    Northeast,
    Southwest,
}

impl Birte {
    fn sail(&self, direction: &'static Direction) -> Birte {
        if self.dir != direction {
            return Birte {
                x: self.x + 1,
                dir: direction,
                ..*self
            };
        }
        match direction {
            Direction::Northeast => Birte {
                x: self.x + 1,
                y: self.y + 1,
                ..*self
            },
            Direction::Southwest => Birte {
                x: self.x + 1,
                y: self.y - 1,
                ..*self
            },
        }
    }
}

fn crossings(fjord: &str) -> u32 {
    let mut crossings = 1;
    let start = fjord.replace("\n", "").find("B").unwrap();
    let lines: Vec<&str> = fjord.lines().collect();
    let line_length = lines[0].chars().count();
    let mut birte = Birte {
        x: start % line_length,
        y: start / line_length,
        dir: &Direction::Southwest,
    };
    while birte.x != line_length - 1 {
        match birte.dir {
            Direction::Northeast => {
                if lines[birte.y + 1].chars().nth(birte.x + 1).unwrap() == '#'
                    || lines[birte.y + 2].chars().nth(birte.x + 1).unwrap() == '#'
                    || lines[birte.y + 3].chars().nth(birte.x + 1).unwrap() == '#'
                {
                    // We need to cross to other direction
                    birte = birte.sail(&Direction::Southwest);
                    crossings += 1;
                    continue;
                }
            }
            Direction::Southwest => {
                if lines[birte.y - 1].chars().nth(birte.x + 1).unwrap() == '#'
                    || lines[birte.y - 2].chars().nth(birte.x + 1).unwrap() == '#'
                    || lines[birte.y - 3].chars().nth(birte.x + 1).unwrap() == '#'
                {
                    // We need to cross to other direction
                    birte = birte.sail(&Direction::Northeast);
                    crossings += 1;
                    continue;
                }
            }
        }
        birte = birte.sail(&birte.dir);
    }
    crossings
}

fn main() {
    let fjord = fs::read_to_string("fjord.txt").unwrap();
    println!("{}", crossings(&fjord));
}

#[test]
fn test_crossings() {
    let fjord = "####################\n\
                 #    ###            \n\
                 #                   \n\
                 #                   \n\
                 #                   \n\
                 #                   \n\
                 #                   \n\
                 #                   \n\
                 #B                  \n\
                 ###    ####     #   \n\
                 ####################";
    assert_eq!(crossings(&fjord), 5);
}

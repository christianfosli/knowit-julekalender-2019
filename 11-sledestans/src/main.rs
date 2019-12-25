use std::fs;

enum TerainType {
    Gress,
    Is(i64),
    Asfalt,
    Skog,
    Fjell { uphill: bool },
}

fn main() {
    let landing_strip = fs::read_to_string("terreng.txt").expect("Unable to read file");
    let start_speed = 10_703_437;
    println!("{}", distance(start_speed, &landing_strip));
}

// Returns the new speed after sliding along one block of this terain
fn slide_along(cur_speed: i64, terain: TerainType) -> i64 {
    match terain {
        TerainType::Gress => cur_speed - 27,
        TerainType::Asfalt => cur_speed - 59,
        TerainType::Skog => cur_speed - 212,
        TerainType::Is(cnt) => cur_speed + 12 * cnt,
        TerainType::Fjell { uphill } => {
            if uphill {
                cur_speed - 70
            } else {
                cur_speed + 35
            }
        }
    }
}

fn distance(start_speed: i64, landing_strip: &str) -> i64 {
    let mut speed = start_speed;
    let mut distance = 0;
    let mut ice_counter = 0;
    let mut mountain_counter = 0;
    for c in landing_strip.chars() {
        distance += 1;
        if c != 'I' {
            ice_counter = 0;
        }
        match c {
            'G' => speed = slide_along(speed, TerainType::Gress),
            'I' => {
                ice_counter += 1;
                speed = slide_along(speed, TerainType::Is(ice_counter));
            }
            'A' => speed = slide_along(speed, TerainType::Asfalt),
            'S' => speed = slide_along(speed, TerainType::Skog),
            'F' => {
                mountain_counter += 1;
                speed = slide_along(
                    speed,
                    TerainType::Fjell {
                        uphill: mountain_counter % 2 != 0,
                    },
                )
            }
            _ => panic!("Unexpected character"),
        }
        if speed < 0 {
            break;
        }
    }
    distance
}

#[test]
fn test_distance() {
    let landing_strip = "IIGGFFAIISGIFFSGFFAAASS";
    let start_speed = 300;
    assert_eq!(distance(start_speed, landing_strip), 11);
}

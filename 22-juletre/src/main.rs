use regex::Regex;
use std::fs;

fn count_meters_of_tree(forest: &str) -> f64 {
    let trees_in_line_re = Regex::new(r"(^|\s)#").unwrap();
    let mut meters_of_tree = 0.0;
    for l in forest.lines() {
        meters_of_tree += trees_in_line_re.find_iter(l).count() as f64 * 0.2;
    }
    (meters_of_tree * 10.0).round() / 10.0
}

fn main() {
    let forest = fs::read_to_string("forest.txt").unwrap();
    let meters = count_meters_of_tree(&forest);
    println!("NOK {}", meters * 200.0);
}

#[test]
fn test_count_meters_of_tree() {
    let forest = "                      #
    #                 ###
   ###               #####
  #####             #######
 #######           #########
   ###            ###########
  #####          #############
 #######              ###
   ###               #####
  #####     #       #######
 #######   ###     #########
   ###    #####   ###########
  #####    ###   #############
 #######  #####       ###
   ###     ###       #####
  #####   #####     #######
 #######   ###     #########
   ###    #####   ###########
  #####    ###   #############
    #       #          #
    #       #          #     ";

    assert_eq!(count_meters_of_tree(&forest), 10.6);
}

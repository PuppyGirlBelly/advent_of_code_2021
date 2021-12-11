use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let lines: Vec<u16> = file_to_vec(String::from("input.txt"));
    let mut increase_count: u16 = 0;
    let mut last_total: u16 = 65535;
    let mut current_total: u16;

    for x in 0..(lines.len() - 2) {
        current_total = lines[x] + lines[x + 1] + lines[x + 2];

        if is_bigger(&current_total, &last_total) {
            increase_count += 1;
            println!("{} (increased)", &current_total);
        } else {
            println!("{}", &current_total);
        }

        last_total = current_total;
    }

    println!("Increases: {}", increase_count);
}

fn is_bigger(x: &u16, y: &u16) -> bool {
    x > y
}

fn line_to_u16(line: String) -> u16 {
    line.parse::<u16>().unwrap()
}

fn file_to_vec(filename: String) -> Vec<u16> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(io::Result::ok)
        .map(|s| line_to_u16(s))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_bigger_big_and_small() {
        assert!(is_bigger(&12, &8));
    }

    #[test]
    fn is_bigger_small_and_big() {
        assert!(!is_bigger(&8, &12));
    }

    #[test]
    fn is_bigger_same_num() {
        assert!(!is_bigger(&8, &8));
    }

    #[test]
    fn line_to_number() {
        assert_eq!(line_to_u16("200".to_string()), 200);
    }

    #[test]
    #[should_panic]
    fn string_to_number_error() {
        line_to_u16("and".to_string());
    }

    #[test]
    fn file_to_u16_vec() {
        assert_eq!(
            file_to_vec(String::from(
                "/mnt/c/Users/SamanthaC/Advent_Of_Code_2021/day1/src/test_input.txt"
            )),
            [100, 200, 300]
        )
    }
}

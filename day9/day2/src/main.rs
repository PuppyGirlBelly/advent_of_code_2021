use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = File::open(String::from("input")).unwrap();
    let reader = BufReader::new(file);
    let mut horizontal_pos: usize = 0;
    let mut depth_pos: usize = 0;
    let mut aim: usize = 0;
    let result: usize;

    for line in reader.lines() {
        let instruction: (String, usize) = line_to_instruction(line.unwrap());

        match instruction.0.as_str() {
            "forward" => {
                             horizontal_pos += instruction.1;
                             depth_pos += instruction.1 * aim;
                         },
            "down" => aim += instruction.1,
            "up" => aim -= instruction.1,
            _ => panic!(),
        }
    }

    result = horizontal_pos * depth_pos;

    println!("horizontal: {}\ndepth: {}\nhorizontal * depth: {}", horizontal_pos, depth_pos, result);
}

fn line_to_instruction(line: String) -> (String, usize) {
    let line_split: Vec<&str> = line.split(' ').collect();

    let x: String = String::from(line_split[0]);
    let y: usize = line_split[1].parse::<usize>().unwrap();

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing() {
        let line = String::from("forward 8");
        assert_eq!( line_to_instruction(line), (String::from("forward"), 8) );
    }
}

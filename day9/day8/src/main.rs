// use std::io::{BufRead, BufReader, Lines};
// use std::fs::File;
use std::time::Instant;
use std::fs::read_to_string;

mod line;
use line::Line;

fn main() {
    let start: Instant = Instant::now();

    let input: Vec<String> = file_to_vec(String::from("src/example_input.txt"));
    let lines: Vec<Line> = input.iter().map(|x| Line::new(x)).collect::<Vec<_>>();
    let mut total: u32 = 0;
    let mut output: u16;

    for mut line in lines {
        line.find_key();
        output = line.decode_output();
        total += output as u32;
        println!("{}",output);
    }

    println!("sum of output: {}", total);

    println!("This main took: {:?}", Instant::now().duration_since(start));
}

fn file_to_vec(input: String) -> Vec<String> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test() {
    }

}

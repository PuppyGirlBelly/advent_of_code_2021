// use std::io::{BufRead, BufReader, Lines};
// use std::fs::File;
use std::time::Instant;
use std::fs::read_to_string;

mod line;

fn main() {
    let start: Instant = Instant::now();

    let input: Vec<_> = file_to_vec(String::from("src/input.txt"));

    println!("known numbers {:?}", input);

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

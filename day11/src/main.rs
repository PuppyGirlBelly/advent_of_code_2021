use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
// use std::fs::read_to_string;
use std::time::Instant;

mod map;
use map::Map;

fn main() {
    let start: Instant = Instant::now();
    let file: File = File::open(String::from("src/input.txt")).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Lines<BufReader<File>> = reader.lines();
    // let mut lines: Peekable<Lines<BufReader<File>>> = reader.lines().peekable();
    // let input: Vec<_> = file_to_vec("src/input.txt");

    for _line in lines {
    }

    println!("This main took: {:?}", Instant::now().duration_since(start));
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test() {
        assert!(true);
    }

}

use std::io::{BufRead, BufReader, Lines};
use std::fs::File;

fn main() {
    let file: File = File::open(String::from("src/input.txt")).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    // let mut lines: Peekable<Lines<BufReader<File>>> = reader.lines().peekable();
    let lines: Lines<BufReader<File>> = reader.lines();

    for _line in lines {
    }

}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test() {
        assert!(true);
    }

}

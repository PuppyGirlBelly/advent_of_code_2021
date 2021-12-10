use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::time::Instant;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let start: Instant = Instant::now();

    let file: File = File::open(String::from("src/example_input.txt")).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines: Peekable<Lines<BufReader<File>>> = reader.lines().peekable();
    let mut tracker: usize = 0;
    let mut stack: Vec<char>;

    for line in lines {
        stack = Vec::new();

        println!("Line: {}", line.as_ref().unwrap());
        'chunk: for chunk in line.unwrap().chars() {
            if "([{<".contains(chunk) {
                stack.push(chunk);
            } else {
                match chunk {
                    '>' if stack.last().unwrap() == &'<' => {stack.pop();},
                    '>' if stack.last().unwrap() != &'<' => {tracker += 25137; println!("{} does not match {}", stack.last().unwrap(), chunk); break 'chunk},
                    '}' if stack.last().unwrap() == &'{' => {stack.pop();},
                    '}' if stack.last().unwrap() != &'{' => {tracker += 1197; println!("{} does not match {}", stack.last().unwrap(), chunk); break 'chunk},
                    '[' if stack.last().unwrap() == &']' => {stack.pop();},
                    '[' if stack.last().unwrap() != &']' => {tracker += 57; println!("{} does not match {}", stack.last().unwrap(), chunk); break 'chunk},
                    '(' if stack.last().unwrap() == &')' => {stack.pop();},
                    '(' if stack.last().unwrap() != &')' => {tracker += 3; println!("{} does not match {}", stack.last().unwrap(), chunk); break 'chunk},
                     _  => {();},
                }
            }
        }
    }

    println!("Total error score: {}", tracker);

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

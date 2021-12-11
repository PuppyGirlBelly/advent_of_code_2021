use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;

fn main() {
    let start: Instant = Instant::now();
    let file: File = File::open(String::from("src/input.txt")).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Lines<BufReader<File>> = reader.lines();
    let mut opening_stack: Vec<char>;
    let mut closing_stack: Vec<char>;
    let mut incomplete_stacks: Vec<String> = Vec::new();
    let mut broken_string: bool;

    for line in lines {
        broken_string = false;
        opening_stack = Vec::new();
        closing_stack = Vec::new();

        // println!("Line: {:?}", line.as_ref().unwrap());
        'chunk: for chunk in line.unwrap().chars() {
            if "([{<".contains(chunk) {
                match chunk {
                    '<' => { opening_stack.push(chunk); closing_stack.push('>'); }
                    '{' => { opening_stack.push(chunk); closing_stack.push('}'); }
                    '[' => { opening_stack.push(chunk); closing_stack.push(']'); }
                    '(' => { opening_stack.push(chunk); closing_stack.push(')'); }
                     _ => {();}
                }
            } else {
                // println!("chunk: {}\nopening_stack: {:?}\nclosing_stack: {:?}", chunk, opening_stack, closing_stack);
                match chunk {
                    '>' if opening_stack.last().unwrap() == &'<' => { opening_stack.pop(); closing_stack.pop(); }
                    '}' if opening_stack.last().unwrap() == &'{' => { opening_stack.pop(); closing_stack.pop(); }
                    ']' if opening_stack.last().unwrap() == &'[' => { opening_stack.pop(); closing_stack.pop(); }
                    ')' if opening_stack.last().unwrap() == &'(' => { opening_stack.pop(); closing_stack.pop(); }
                    '>' if opening_stack.last().unwrap() != &'<' => { broken_string = true; }
                    '}' if opening_stack.last().unwrap() != &'{' => { broken_string = true; }
                    ']' if opening_stack.last().unwrap() != &'[' => { broken_string = true; }
                    ')' if opening_stack.last().unwrap() != &'(' => { broken_string = true; }
                    _ => { (); }
                }
            }
        }

        if !broken_string {
            incomplete_stacks.push(closing_stack.iter().rev().collect::<String>());
        }
    }

    let mut scores: Vec<usize> = Vec::new(); 
    let mut score: usize;

    for stack in &incomplete_stacks {
        score = 0;

        // println!("Line: {:?}", line.as_ref().unwrap());
        for chunk in stack.chars() {
            match chunk {
                ')' => { score *= 5; score += 1; }
                ']' => { score *= 5; score += 2; }
                '}' => { score *= 5; score += 3; }
                '>' => { score *= 5; score += 4; }
                 _ => {();}
            }
        }

        scores.push(score);
    }
    scores.sort();

    println!("Winning Score: {:?}", scores[scores.len()/2]);

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

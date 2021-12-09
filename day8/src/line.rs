use std::{collections::HashMap, str::Chars};
use std::iter::FromIterator;

pub struct Line {
    encoded_digits: Vec<String>,
    output: Vec<String>,
    segment_key: HashMap<char, char>,
}

impl Line{
    pub fn new(input: String) -> Line {
        let mut return_line = Line {
            encoded_digits: string_trim_encoded_key(&input),
            output: string_trim_output(&input),
            segment_key: HashMap::new(),
        };
        
        return_line.segment_key.insert('a', 'a');
        return_line.segment_key.insert('b', 'b');
        return_line.segment_key.insert('c', 'c');
        return_line.segment_key.insert('d', 'd');
        return_line.segment_key.insert('e', 'e');
        return_line.segment_key.insert('f', 'f');
        return_line.segment_key.insert('g', 'g');

        return_line
    }
    

    pub fn find_key(&mut self) {
        let one:   &str = self.encoded_digits.iter().filter(|x| x.len() == 2).collect::<Vec<_>>()[0];
        println!("one: {}", one);
        let four:  &str = self.encoded_digits.iter().filter(|x| x.len() == 4).collect::<Vec<_>>()[0];
        println!("four: {}", four);
        let seven: &str = self.encoded_digits.iter().filter(|x| x.len() == 3).collect::<Vec<_>>()[0];
        println!("seven: {}", seven);
        let eight: &str = self.encoded_digits.iter().filter(|x| x.len() == 7).collect::<Vec<_>>()[0];
        println!("eight: {}", eight);

        let encoded_digits: String = self.encoded_digits.join(" ");
        let mut counter: usize;

        for segment in "abcdefg".chars() {
            counter = encoded_digits.chars().filter(|c| *c == segment).collect::<String>().len();
            println!("segment: {} | one: {} | four: {:5} | seven: {:5} | eight: {:5} | counter: {:5}", 
                segment,
                counter,
                one.contains(segment), 
                four.contains(segment), 
                seven.contains(segment), 
                eight.contains(segment), 
            );

        //    a:e
        //    b:c
        //    c:d
        //    d:a
        //    e:f
        //    f:d
        //    g:b
    // Segment rules:
    // a: in(7,8) appears(8)
    // b: in(4,8) appears(6)
    // c: in(1,4,7,8) appears(8)
    // d: in(4,8) appears(7)
    // e: in(8) appears(4)
    // f: in(1,4,7,8) appears(9)
    // g: in(8) appears(7)
            if counter == 8 && !one.contains(segment) && !four.contains(segment) && seven.contains(segment) && eight.contains(segment) {
                *self.segment_key.get_mut(&segment).unwrap() = 'a';
                println!("{}:a",segment);
            } else if counter == 6 && !one.contains(segment) && four.contains(segment) && !seven.contains(segment) && eight.contains(segment) {
                *self.segment_key.get_mut(&segment).unwrap() = 'b';
                println!("{}:b",segment);
            } else if counter == 8 && one.contains(segment) && four.contains(segment) && seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'c';
                println!("{}:c",segment);
            } else if counter == 7 && !one.contains(segment) && four.contains(segment) && !seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'd';
                println!("{}:d",segment);
            } else if counter == 4 && !one.contains(segment) && !four.contains(segment) && !seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'e';
                println!("{}:e",segment);
            } else if counter == 9 && one.contains(segment) && four.contains(segment) && seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'f';
                println!("{}:f",segment);
            } else if counter == 7 && !one.contains(segment) && !four.contains(segment) && !seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'g';
                println!("{}:g",segment);
            } else {
                panic!()
            }
        }
        // *self.segment_key.get_mut(&encoded_8.next().unwrap()).unwrap() = 'g';
    }

    // pub fn decode_string(&self, input: String) -> String {
    //     input.chars()
    //         .map(|c| self.segment_key[&c])
    //         .collect::<String>()
    // }

    // pub fn decode_output(&mut self) -> Vec<String> {
    //     self.output.iter()
    //         .map(|x|)
    // }
}

fn string_trim_output(input: &str) -> Vec<String> {
    input.get((input.find('|').unwrap() + 2)..)
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn string_trim_encoded_key(input: &str) -> Vec<String> {
    input.get(..(input.find('|').unwrap() - 1))
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn line_new_test() {
        let input: String = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        let line: Line = Line::new(input);
        let encoded_digits: Vec<String> = ["be","cfbegad","cbdgef","fgaecd","cgeb","fdcge","agebfd","fecdb","fabcd","edb"].iter().map(|x| x.to_string()).collect();
        let output: Vec<String> = ["fdgacbe","cefdb","cefbgd","gcbe"].iter().map(|x| x.to_string()).collect();

        assert_eq!(line.encoded_digits, encoded_digits);
        assert_eq!(line.output, output);
        assert_eq!(line.segment_key[&'a'], 'a' );
    }

    #[test]
    fn test_find_key() {
        let input: String = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        let mut line: Line = Line::new(input);

        line.find_key();

        assert_eq!(line.segment_key[&'a'], 'e');
        assert_eq!(line.segment_key[&'b'], 'c');
        assert_eq!(line.segment_key[&'c'], 'd');
        assert_eq!(line.segment_key[&'d'], 'a');
        assert_eq!(line.segment_key[&'e'], 'f');
        assert_eq!(line.segment_key[&'f'], 'g');
        assert_eq!(line.segment_key[&'g'], 'b');
    }

    // #[test]
    // fn test_decode_string() {
    //     let input: String = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
    //     let mut line: Line = Line::new(input);

    //     line.find_key();

    //     assert_eq!(line.decode_string(line.encoded_digits[0]), 'fg');
    // }

}

use std::{collections::HashMap, str::Chars};
use std::iter::{Iterator, FromIterator};

pub struct Line {
    encoded_digits: Vec<String>,
    output: Vec<String>,
    segment_key: HashMap<char, char>,
    digit_key: HashMap<String, u16>,
}

impl Line{
    pub fn new(input: &str) -> Line {
        let mut return_line = Line {
            encoded_digits: string_trim_encoded_key(input),
            output: string_trim_output(input),
            segment_key: HashMap::new(),
            digit_key: HashMap::new(),
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
        let four:  &str = self.encoded_digits.iter().filter(|x| x.len() == 4).collect::<Vec<_>>()[0];
        let seven: &str = self.encoded_digits.iter().filter(|x| x.len() == 3).collect::<Vec<_>>()[0];
        let eight: &str = self.encoded_digits.iter().filter(|x| x.len() == 7).collect::<Vec<_>>()[0];

        let encoded_digits: String = self.encoded_digits.join(" ");
        let mut counter: usize;

        for segment in "abcdefg".chars() {
            counter = encoded_digits.chars().filter(|c| *c == segment).collect::<String>().len();
            
            if counter == 8 && !one.contains(segment) && !four.contains(segment) && seven.contains(segment) && eight.contains(segment) {
                *self.segment_key.get_mut(&segment).unwrap() = 'a';
            } else if counter == 6 && !one.contains(segment) && four.contains(segment) && !seven.contains(segment) && eight.contains(segment) {
                *self.segment_key.get_mut(&segment).unwrap() = 'b';
            } else if counter == 8 && one.contains(segment) && four.contains(segment) && seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'c';
            } else if counter == 7 && !one.contains(segment) && four.contains(segment) && !seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'd';
            } else if counter == 4 && !one.contains(segment) && !four.contains(segment) && !seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'e';
            } else if counter == 9 && one.contains(segment) && four.contains(segment) && seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'f';
            } else if counter == 7 && !one.contains(segment) && !four.contains(segment) && !seven.contains(segment) && eight.contains(segment){
                *self.segment_key.get_mut(&segment).unwrap() = 'g';
            } else {
                panic!()
            }
        }
    }

    pub fn decode_string(&self, input: &str) -> String {
        input.chars()
            .map(|c| self.segment_key[&c])
            .collect::<String>()
    }

    pub fn decode_number(&self, input: &str) -> u16 {
        let number: String = sort_string(&self.decode_string(input));

        match number.as_str() {
            "abcefg"  => 0,  
            "cf"      => 1,
            "acdeg"   => 2,
            "acdfg"   => 3,
            "bcdf"    => 4,
            "abdfg"   => 5,
            "abdefg"  => 6,
            "acf"     => 7,
            "abcdefg" => 8,
            "abcdfg"  => 9,
            _         => panic!()
        }
    }

    pub fn decode_output(&mut self) -> u16 {
        self.output.iter()
            .map(|x| self.decode_number(&x.to_string()))
            .collect::<Vec<u16>>()
            .iter()
            .fold(0, |acc, elem| acc * 10 + elem)
    }
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

fn sort_string(input: &str) -> String {
    let mut output: Vec<char> = input.chars().collect();
    output.sort_by(|a, b| a.cmp(b));
    output.iter().collect::<String>()
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn line_new_test() {
        let input: String = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        let line: Line = Line::new(&input);
        let encoded_digits: Vec<String> = ["be","cfbegad","cbdgef","fgaecd","cgeb","fdcge","agebfd","fecdb","fabcd","edb"].iter().map(|x| x.to_string()).collect();
        let output: Vec<String> = ["fdgacbe","cefdb","cefbgd","gcbe"].iter().map(|x| x.to_string()).collect();

        assert_eq!(line.encoded_digits, encoded_digits);
        assert_eq!(line.output, output);
        assert_eq!(line.segment_key[&'a'], 'a' );
    }

    #[test]
    fn test_find_key() {
        let input: String = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        let mut line: Line = Line::new(&input);

        line.find_key();

        assert_eq!(line.segment_key[&'a'], 'e');
        assert_eq!(line.segment_key[&'b'], 'c');
        assert_eq!(line.segment_key[&'c'], 'd');
        assert_eq!(line.segment_key[&'d'], 'a');
        assert_eq!(line.segment_key[&'e'], 'f');
        assert_eq!(line.segment_key[&'f'], 'g');
        assert_eq!(line.segment_key[&'g'], 'b');
    }

    #[test]
    fn test_decode_string() {
        let input: String = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        let mut line: Line = Line::new(&input);

        line.find_key();

        assert_eq!(line.decode_string(&line.encoded_digits[0]), "cf");
        assert_eq!(line.decode_string(&line.encoded_digits[1]), "dgcfbea");
        assert_eq!(line.decode_string(&line.encoded_digits[4]), "dbfc");
    }

    #[test]
    fn test_sort_string() {
        assert_eq!(sort_string("cfbegad"), "abcdefg");
    }

    #[test]
    fn test_decode_output() {
        let input: String = String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
        let mut line: Line = Line::new(&input);

        line.find_key();

        assert_eq!(line.decode_number(&line.output[0]), 5);
        assert_eq!(line.decode_number(&line.output[1]), 3);
        assert_eq!(line.decode_number(&line.output[2]), 5);
        assert_eq!(line.decode_number(&line.output[3]), 3);
        assert_eq!(line.decode_output(), 5353)
    }
}

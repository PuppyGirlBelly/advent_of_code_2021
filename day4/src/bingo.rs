use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::iter::Peekable;

use crate::{string_to_vec_u16, is_all_same}; 

#[derive(Debug, Clone)]
pub struct Bingo {
    pub board: Vec<u16>,
}

impl Bingo {
    pub fn new() -> Bingo {
        Bingo {
            board: vec![],
        }
    }

    pub fn populate(&mut self, lines: &mut Peekable<Lines<BufReader<File>>>) {
        let mut line: String;

        for _ in 0..5 {
            line = lines.next().unwrap().unwrap();
            let line_vec = string_to_vec_u16(line, " ");
            self.board.extend(line_vec);
        }
    }

    pub fn check_num(&mut self, number_called: u16) {
        for i in 0..self.board.len() {
            if self.board[i] == number_called { self.board[i] = 100 };
        }
    }

    //  0  1  2  3  4
    //  5  6  7  8  9
    // 10 11 12 13 14
    // 15 16 17 18 19
    // 20 21 22 23 24
    pub fn check_bingo(&mut self) -> bool {
        for i in (0..25).step_by(5) {
            if is_all_same(&self.board[i..i+5]) {
                return true;
            }
        }

        let mut temp_slice: Vec<u16>;
        for i in 0..5 {
            temp_slice = vec![self.board[i],
                              self.board[i+5],
                              self.board[i+10],
                              self.board[i+15],
                              self.board[i+20]];
            if is_all_same(&temp_slice) {
                return true;
            }
        }

        return false;
    }

    pub fn tally_score(&self, win_num: u16) -> usize {
        let mut sum: usize = 0;

        for i in 0..self.board.len() {
            if self.board[i] != 100 {
                sum += self.board[i] as usize;
            }
        }

        sum * win_num as usize
    }
    pub fn display_card(&self) {
        println!("{:3} {:3} {:3} {:3} {:3} ", self.board[0], self.board[1], self.board[2], self.board[3], self.board[4]);
        println!("{:3} {:3} {:3} {:3} {:3} ", self.board[5], self.board[6], self.board[7], self.board[8], self.board[9]);
        println!("{:3} {:3} {:3} {:3} {:3} ", self.board[10], self.board[11], self.board[12], self.board[13], self.board[14]);
        println!("{:3} {:3} {:3} {:3} {:3} ", self.board[15], self.board[16], self.board[17], self.board[18], self.board[19]);
        println!("{:3} {:3} {:3} {:3} {:3} ", self.board[20], self.board[21], self.board[22], self.board[23], self.board[24]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bingo_test() {
        let mut bingo: Bingo = Bingo::new();
        let file: File = File::open(String::from("src/test_input.txt")).unwrap();
        let reader: BufReader<File> = BufReader::new(file);
        let mut lines: Peekable<Lines<BufReader<File>>> = reader.lines().peekable();

        let solution = vec![78,27,82,68,20,14,2,34,51,7,58,57,99,37,81,9,4,0,76,45,67,69,70,17,23];
         
        bingo.populate(&mut lines);
        assert_eq!(bingo.board, solution);
    }

    #[test]
    fn bingo_num_test() {
        let mut bingo: Bingo = Bingo::new();
        let file: File = File::open(String::from("src/test_input.txt")).unwrap();
        let reader: BufReader<File> = BufReader::new(file);
        let mut lines: Peekable<Lines<BufReader<File>>> = reader.lines().peekable();
        bingo.populate(&mut lines);

        //78 , 27 , 82 , 68 , 20 ,
        //14 ,  2 , 34 , 51 ,  7 ,
        //58 , 57 , 99 , 37 , 81 ,
         //9 ,  4 ,  0 , 76 , 45 ,
        //67 , 69 , 70 , 17 , 23
        let solution = vec![78,27,100,68,20,14,2,100,51,7,58,57,100,37,81,9,4,100,76,45,67,69,100,17,23];
        bingo.check_num(82);
        bingo.check_num(34);
        bingo.check_num(99);
        bingo.check_num(0);
        bingo.check_num(70);
         
        // assert_eq!(bingo.board.len(), 25);
        // assert!(bingo.check_bingo());
        assert_eq!(bingo.board, solution);
    }

    #[test]
    fn bingo_check() {
        let mut bingo: Bingo = Bingo::new();
        let file: File = File::open(String::from("src/test_bingo_input.txt")).unwrap();
        let reader: BufReader<File> = BufReader::new(file);
        let mut lines: Peekable<Lines<BufReader<File>>> = reader.lines().peekable();

        // let solution = vec![78,27,82,68,20,14,2,34,51,7,58,57,99,37,81,9,4,0,76,45,67,69,70,17,23];
         
        bingo.populate(&mut lines);
        assert!(bingo.check_bingo());
    }
}

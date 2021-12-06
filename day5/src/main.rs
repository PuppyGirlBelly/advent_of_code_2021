use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use text_io::scan;

mod board;
use board::Board;
// use std::iter::Peekable;

// - 2D array of 1000x1000 'points'
// - Parse lines in input as two tuples, representing coordinates
// - Turn two tuples into a range of coordinates
// - use function to return a vector of coordinates
// - iterate over coordinates vector to increment points on 2D array
// - iterate over 2D array and increment counter to count points that 
//     display a number greater than 2


fn main() {
    let file: File = File::open(String::from("src/input.txt")).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    // let mut lines: Peekable<Lines<BufReader<File>>> = reader.lines().peekable();
    let lines: Lines<BufReader<File>> = reader.lines();
    let mut coords: Vec<(usize, usize)>;
    let mut line_range: Vec<(usize, usize)>;
    let mut board: Board = Board::new();

    for line in lines {
        coords = line_to_coords(line.unwrap());
        // if coords[0].0 == coords[1].0 || coords[0].1 == coords[1].1 {
        line_range = coords_to_range(coords);
        board.line_increment(line_range);
        // }
    }

    println!("board: {:?}\ntotal intersections: {}", board.map, board.total_intersections());
}

fn line_to_coords(line: String) -> Vec<(usize, usize)> {
    let (x1, y1, x2, y2): (usize, usize, usize, usize);

    scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);

    return vec![(x1, y1), (x2, y2)]
}

fn coords_to_range(input: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let (mut x1, mut y1) = input[0];
    let (mut x2, mut y2) = input[1];
    let mut result: Vec<(usize, usize)> = Vec::new();

    if x1 == x2 || y1 == y2 {
        if x1 > x2 { std::mem::swap(&mut x1, &mut x2) };
        if y1 > y2 { std::mem::swap(&mut y1, &mut y2) };

        for x in x1..=x2 {
            for y in y1..=y2 {
                result.push( (x, y) );
            }
        }
    } else {
        let x_range: Vec<usize>;
        let y_range: Vec<usize>;

        if x1 > x2 { 
            std::mem::swap(&mut x1, &mut x2);
            x_range = (x1..=x2).rev().collect();
        } else {
            x_range = (x1..=x2).collect();
        }

        if y1 > y2 { 
            std::mem::swap(&mut y1, &mut y2);
            y_range = (y1..=y2).rev().collect();
        } else {
            y_range = (y1..=y2).collect();
        }

        for i in 0..x_range.len() {
            result.push( (x_range[i], y_range[i]) );
        }
    }
    
    return result;
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn input_parsing() {
        let input1: String = String::from("5,1 -> 0,1");
        assert_eq!(line_to_coords(input1), vec![(5,1),(0,1)]);
        let input1: String = String::from("0,0 -> 5,5");
        assert_eq!(line_to_coords(input1), vec![(0,0),(5,5)]);
    }

    #[test]
    fn input_diagonal() {
        let input: String = String::from("9,7 -> 7,9");
        let input_coord: Vec<(usize, usize)> = line_to_coords(input);
        let input_range: Vec<(usize, usize)> = coords_to_range(input_coord);
        let solution: Vec<(usize, usize)> = vec![(9,7), (8,8), (7,9)];
        
        assert_eq!(input_range, solution);
    }
}

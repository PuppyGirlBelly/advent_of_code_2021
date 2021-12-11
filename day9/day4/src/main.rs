use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::iter::Peekable;

mod bingo;
use bingo::Bingo;

fn main() {
    let file: File = File::open(String::from("src/input.txt")).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines: Peekable<Lines<BufReader<File>>> = reader.lines().peekable();
    
    let bingo_numbers = string_to_vec_u16( lines.next().unwrap().unwrap(), "," );
    let mut bingo_cards: Vec<Bingo> = vec![];
    let mut new_card: Bingo;
    let mut win_card: Bingo = Bingo::new();
    let mut win_num: u16 = 0;
    let mut cards_to_remove: Vec<usize> = Vec::new();

    while lines.peek().is_some() {
        lines.next();
        new_card = Bingo::new();
        new_card.populate(&mut lines);
        bingo_cards.push(new_card);
    }

    for num_drawn in bingo_numbers {
        cards_to_remove = Vec::new();

        for i in 0..bingo_cards.len() {
            bingo_cards[i].check_num(num_drawn);
            if bingo_cards[i].check_bingo() {
                win_card = bingo_cards[i].clone();
                win_num = num_drawn;
                cards_to_remove.push(i);
                win_card.display_card();
                println!("Win Num: {}", win_num);
                println!("Total Score: {}", win_card.tally_score(win_num));
            }
        }

        cards_to_remove.reverse();
        for card in cards_to_remove {
            bingo_cards.remove(card);
        }
    }

}

fn string_to_vec_u16(input: String, separator: &str) -> Vec<u16> {
    input.split(separator)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u16>().unwrap())
        .collect()
}

fn is_all_same(array: &[u16]) -> bool {
    array.iter().min() == array.iter().max()
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn string_vec_u16_comma() {
        let input: String = String::from("1,2,3,4,5");
        let result: Vec<u16> = string_to_vec_u16(input, ",");
        assert_eq!(result, [1,2,3,4,5]);
    }

    #[test]
    fn string_vec_u16_space() {
        let input: String = String::from("1 2 3 4 5");
        let result: Vec<u16> = string_to_vec_u16(input, " ");
        assert_eq!(result, [1,2,3,4,5]);
    }

    #[test]
    fn vec_all_same() {
        let input_true: Vec<u16> = vec![4,4,4,4,4];
        let input_false: Vec<u16> = vec![4,4,3,4,4];
        assert!(is_all_same(&input_true));
        assert!(!is_all_same(&input_false));
    }
}

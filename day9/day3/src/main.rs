use std::io::{self, BufRead, BufReader};
use std::fs::File;

fn main() {
    // let file = File::open(String::from("input.txt")).unwrap();
    let file = File::open(String::from("input.txt")).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    let oxygen = oxygen_filter(lines, 0);
    let oxygen_ans = isize::from_str_radix(&oxygen, 2).unwrap();
    println!("oxygen: {:?}", oxygen_ans);

    let file = File::open(String::from("input.txt")).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    let co2 = co2_filter(lines, 0);
    let co2_ans = isize::from_str_radix(&co2, 2).unwrap();
    println!("co2: {:?}", co2_ans);

    println!("anser: {}", co2_ans*oxygen_ans);
    
    // let mut line_num: u32 = 0;
    // let mut bit_counter: Vec<u32> = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    // let gamma: u32;
    // let epsilon: u32;

    // for line in reader.lines() {
    //     line_num += 1;
        
    //     let line_vec = line_to_vec(line.unwrap());
    //     bit_counter = increment_vec(bit_counter, line_vec);
    // }

    // gamma = binary_to_decimal(vec_to_binary(&bit_counter, line_num/2));
    // epsilon = binary_to_decimal(bit_flip(vec_to_binary(&bit_counter, line_num/2)));

    // println!("bit_counter: {:?}", bit_counter);
    // println!("gamma: {:?}", gamma);
    // println!("epsilon: {:?}", epsilon);
    // println!("answer: {}", gamma*epsilon);
}

fn oxygen_filter(nums: Vec<String>, i: usize) -> String {
    if nums.len() == 1 {
        let result = String::from(&nums[0]);
        return result;
    }

    let mut one_counter: usize = 0;
    let mut zero_counter: usize = 0;
    let filter: char;
    let mut filtered_nums: Vec<String> = Vec::new();

    for line in &nums {
        if line.chars().nth(i).unwrap() == '1' {
            one_counter += 1;
        } else {
            zero_counter += 1;
        }
    }

    if one_counter >= zero_counter {
        filter = '1';
    } else {
        filter = '0';
    }

    for line in &nums {
        if line.chars().nth(i).unwrap() == filter {
            filtered_nums.push(String::from(line))
        }
    }

    oxygen_filter(filtered_nums, i+1)
}

fn co2_filter(nums: Vec<String>, i: usize) -> String {
    if nums.len() == 1 {
        let result = String::from(&nums[0]);
        return result;
    }

    let mut one_counter: usize = 0;
    let mut zero_counter: usize = 0;
    let filter: char;
    let mut filtered_nums: Vec<String> = Vec::new();

    for line in &nums {
        if line.chars().nth(i).unwrap() == '1' {
            one_counter += 1;
        } else {
            zero_counter += 1;
        }
    }

    if zero_counter <= one_counter {
        filter = '0';
    } else {
        filter = '1';
    }

    for line in &nums {
        if line.chars().nth(i).unwrap() == filter {
            filtered_nums.push(String::from(line))
        }
    }

    co2_filter(filtered_nums, i+1)
}

fn line_to_vec(line: String) -> Vec<u32> {
    line.chars().map(|x| x.to_digit(10).unwrap()).collect()    
}

fn increment_vec(source_vec: Vec<u32>, incr_vec: Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for (i, x) in source_vec.iter().enumerate() {
        let x = x + incr_vec[i];
        result.push(x);
    }

    result
}

fn vec_to_binary(source: &Vec<u32>, threshold: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for x in source {
        if x >= &threshold {
            result.push(1);
        } else {
            result.push(0);
        }
    }

    result
}

fn bit_flip(source: Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for x in source {
        if x == 1 {
            result.push(0);
        } else {
            result.push(1);
        }
    }

    result
}

fn binary_to_decimal(source: Vec<u32>) -> u32 {
    source.iter().fold(0, |acc, &b| acc*2 + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_to_u32_vec() {
        let line = line_to_vec(String::from("100110"));
        assert_eq!(line, [1,0,0,1,1,0]);
    }

    #[test]
    fn incrementing_two_vecs() {
        let source: Vec<u32> = vec![1,0,0,1,1,0];
        let increm: Vec<u32> = vec![1,0,1,0,0,0];
        let result: Vec<u32> = increment_vec(source, increm);

        assert_eq!(result, [2,0,1,1,1,0]);
    }

    #[test]
    fn vec_to_binary_test() {
        let correct_vector = vec![0,0,0,1,1,0,0,0,1,0,1,0];
        let test_vector = vec![498, 483, 484, 509, 508, 495, 491, 497, 507, 495, 505, 498];
        let result = vec_to_binary(&test_vector, 500);
        assert_eq!(result, correct_vector);
    }

    #[test]
    fn bit_flip_test() {
        assert_eq!(bit_flip(vec![0,0,0,1,1,0,0,0,1,0,1,0]), [1,1,1,0,0,1,1,1,0,1,0,1])
    }

    #[test]
    fn binary_to_decimal_test() {
        let result = binary_to_decimal(vec![0,0,0,1,1,0,0,0,1,0,1,0]);
        assert_eq!(result, 394);
    }
}

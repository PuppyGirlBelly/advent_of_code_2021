use std::fs::read_to_string;

fn main() {
    let start = std::time::Instant::now();
    let input: Vec<i32> = file_to_vec("src/input.txt");

    let result: i32 = *(0..=*input.iter().max().unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .map(|x| distances(&input, x.clone()))
        .collect::<Vec<i32>>()
        .iter()
        .min()
        .unwrap();
    
    println!("Fuel consumed: {}", result);
    println!("Benchmark: {:?}",  std::time::Instant::now().duration_since(start));
}

fn file_to_vec(filename: &str) -> Vec<i32> {
    read_to_string(filename)
		.unwrap()
		.trim()
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect()
}

fn distances(positions: &[i32], target: i32) -> i32 {
    positions.iter()
        .map(|x| {
            let dist = (x - target).abs();
            dist * (dist + 1) / 2
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn distance_test() {
        assert_eq!(distance(2, 4), 2);
        assert_eq!(distance(4, 2), 2);
        assert_eq!(distance(0, 5), 5);
        assert_eq!(distance(9, 2), 7);
    }

    #[test]
    fn range_sum_test() {
        assert_eq!(sum_of_range(11), 66);
        assert_eq!(sum_of_range(4), 10);
        assert_eq!(sum_of_range(3), 6);
        assert_eq!(sum_of_range(5), 15);
    }
}

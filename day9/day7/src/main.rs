use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start: Instant = Instant::now();
    let positions: Vec<_> = file_to_vec("src/input.txt");
    // (index of smallest, amount of fuel spent)
    let mut smallest: (usize, usize) = ( 0, usize::MAX );
    let mut fuel_consumption: usize;

    for pos in 0..positions.len() {
        fuel_consumption = 0;

        for i in 0..positions.len() {
            fuel_consumption += sum_of_range(distance(positions[i], pos));
        }

        if fuel_consumption < smallest.1 {
            smallest = (pos, fuel_consumption)
        }
    }

    println!("Position: {}\nFuel consumed: {}", smallest.0, smallest.1);
    println!("This main took: {:?}", Instant::now().duration_since(start));
}

fn file_to_vec(filename: &str) -> Vec<usize> {
    read_to_string(filename)
		.unwrap()
		.trim()
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect()
}

fn distance(position: usize, target: usize) -> usize {
    if position >= target {
        position - target
    } else {
    target - position
    }
}

fn sum_of_range(end: usize) -> usize {
    (0..=end).fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn distance_test() {
        assert_eq!(distance(2,4), 2);
        assert_eq!(distance(4,2), 2);
        assert_eq!(distance(0,5), 5);
        assert_eq!(distance(9,2), 7);
    }

    #[test]
    fn range_sum_test() {
        assert_eq!(sum_of_range(3), 6);
        assert_eq!(sum_of_range(5), 15);
    }
}

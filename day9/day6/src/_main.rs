use std::fs::read_to_string;

fn main() {
    let input: Vec<usize> = file_to_vec("src/input.txt");
    let mut counters: Vec<usize> = vec![0; 9];
    let iterations: usize = 256;

    println!("Initial state: {:?}", input);
    for entry in input {
        counters[entry] += 1;
    }
    println!("Counters: {:?}", counters);

    for i in 0..iterations {
        counters[7] += counters[0];
        counters.push(counters[0]);
        counters.remove(0);

        println!("Day {}: {:?}", i, counters);
    }

    let sum: usize = counters.iter().sum();

    println!("Total Fish: {}", sum);
}

fn file_to_vec(filename: &str) -> Vec<usize> {
    read_to_string(filename)
		.unwrap()
		.trim()
		.split(',')
		.map(|x| x.parse().unwrap())
		.collect()
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_input_parsing() {
        let input: Vec<usize> = file_to_vec("src/example_input.txt");
        assert_eq!(input, vec![3,4,3,1,2]);
    }
}

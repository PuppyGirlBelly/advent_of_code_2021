use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start: Instant = Instant::now();
    let input: Vec<_> = file_to_vec("src/input.txt");

    let mut counters: Vec<_> = (0..9).collect::<Vec<_>>().iter()
        .map(|i| input.iter().filter(|x| **x == *i).count() )
        .collect();

    for _ in 0..256 {
        counters[7] += counters[0];
        counters.push(counters[0]);
        counters.remove(0);
    }

    println!("sum: {:?}", counters.iter().sum::<usize>());

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

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test() {
        assert!(true);
    }

}

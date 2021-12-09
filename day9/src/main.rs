use std::fs::read_to_string;
use std::time::Instant;

mod map;
use map::Map;

fn main() {
    let start: Instant = Instant::now();
    let input: String = read_to_string("src/input.txt").unwrap();
    let mut height_map: Map = Map::new(input);
    let mut sizes: Vec<usize> = height_map.get_basin_sizes();
    sizes.sort();

    let three_biggest: usize = sizes.iter().rev().take(3).product();

    println!("Three biggest iterator's sum: {}", &three_biggest);

    println!("This main took: {:?}", Instant::now().duration_since(start));
}


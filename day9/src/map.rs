type Pos = (usize, usize);

#[derive(Debug, Clone)]
pub struct Map {
    pub point: Vec<usize>,
    low_points: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(input: String) -> Map {
        let mut result = Map {
            point: Vec::new(),
            low_points: Vec::new(),
            width: input.lines().next().unwrap().len() + 2,
            height: input.lines().count() + 2,
        };
        let input_string = "9".repeat(result.width + 1) + &input + &"9".repeat(result.width + 1);
        result.point = string_to_vec(&input_string);
        result.low_points = result.get_low_points();

        result
    }

    pub fn get_surrounding(&self, x: usize, y: usize) -> (usize, usize, usize, usize, usize) {
        let up: usize    = self[(x, y-1)];
        let down: usize  = self[(x, y+1)];
        let left: usize  = self[(x-1, y)];
        let right: usize = self[(x+1, y)];

        (self[(x,y)], up, right, down, left)       
    }

    fn get_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points: Vec<(usize, usize)> = Vec::new();
        let mut points: (usize, usize, usize, usize, usize);

        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                points = self.get_surrounding(x, y);
                if is_smallest(points.0, points.1, points.2, points.3, points.4) {
                    low_points.push((x, y));
                }
            }
        }
        
        low_points
    }

    pub fn recursive_basin_fill(&mut self, x: usize, y: usize) -> usize {
        let mut total = 0;

        if self[(x,y)] == 9 {
            return total;
        }

        self[(x,y)] = 9;

        total += self.recursive_basin_fill(x, y+1);
        total += self.recursive_basin_fill(x+1, y);
        total += self.recursive_basin_fill(x, y-1);
        total += self.recursive_basin_fill(x-1, y);

        return total + 1;
    }

    pub fn get_basin_sizes(&mut self) -> Vec<usize> {
        let mut results: Vec<usize> = Vec::new();
        let mut result: usize;
        let mut point: (usize, usize);

        for i in 0..self.low_points.len() {
            point = self.low_points[i];
            result = self.recursive_basin_fill(point.0, point.1);
            results.push(result);
        }

        results
    }

}

impl std::ops::Index<Pos> for Map {
    type Output = usize;
    fn index(&self, (x, y): Pos) -> &usize {
        &self.point[(y * self.width + x)]
    }
}

impl std::ops::IndexMut<Pos> for Map {
    fn index_mut(&mut self, (x, y): Pos) -> &mut Self::Output {
        &mut self.point[(y * self.width + x)]
    }
}

fn string_to_vec(input: &str) -> Vec<usize> {
    input.replace("\n", "99")
         .split("")
         .filter(|&x| !x.is_empty())
         .map(|x| x.parse::<usize>().unwrap() )
         .collect::<Vec<usize>>()
}

fn is_smallest( x: usize, up: usize, right: usize, down: usize, left: usize ) -> bool {
    x < up && x < right && x < down && x < left
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_low_point() {
        assert!(is_smallest(1, 2, 3, 4, 5));
        assert!(!is_smallest(3, 3, 3, 3, 3));
        assert!(!is_smallest(3, 3, 4, 4, 4));
        assert!(!is_smallest(3, 1, 1, 1, 1));
    }

    #[test]
    fn test_surrounds() {
        // 9 9 9 9 9
        // 9 1 2 3 9
        // 9 4 5 6 9
        // 9 7 8 9 9
        // 9 9 9 9 9
        let input: String = "123\n456\n789\n".to_string();
        let map: Map = Map::new(input);
        println!("Map: {:?}", map.point);

        assert_eq!(map.get_surrounding(1,1),(1,9,2,4,9));
        assert_eq!(map.get_surrounding(2,1),(2,9,3,5,1));
        assert_eq!(map.get_surrounding(3,1),(3,9,9,6,2));
        assert_eq!(map.get_surrounding(1,2),(4,1,5,7,9));
        assert_eq!(map.get_surrounding(2,2),(5,2,6,8,4));
        assert_eq!(map.get_surrounding(3,2),(6,3,9,9,5));
        assert_eq!(map.get_surrounding(1,3),(7,4,8,9,9));
        assert_eq!(map.get_surrounding(2,3),(8,5,9,9,7));
        assert_eq!(map.get_surrounding(3,3),(9,6,9,9,8));
    }

    #[test]
    fn test_get_low_points() {
        use std::fs::read_to_string;
        let input: String = read_to_string("src/example_input.txt").unwrap();
        let height_map: Map = Map::new(input);

        assert_eq!(height_map.low_points, [(2, 1), (10, 1), (3, 3), (7, 5)]);
    }

    #[test]
    fn test_basin_fill() {
        use std::fs::read_to_string;
        let input: String = read_to_string("src/example_input.txt").unwrap();
        let mut height_map: Map = Map::new(input);
        let low_point: (usize, usize) = height_map.low_points[3];

        assert_eq!(height_map.recursive_basin_fill(low_point.0, low_point.1), 9);
    }

    #[test]
    fn test_basin_sizes() {
        use std::fs::read_to_string;
        let input: String = read_to_string("src/example_input.txt").unwrap();
        let mut height_map: Map = Map::new(input);
        let sizes: Vec<usize> = height_map.get_basin_sizes();

        assert_eq!(sizes, [3, 9, 14, 9]);
    }
}

type Pos = (usize, usize);

#[derive(Debug, Clone)]
pub struct Board {
    pub map: Vec<usize>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            // map: vec![0; (MAP_HEIGHT * MAP_WIDTH) as usize],
            map: vec![0; (1000 * 1000) as usize],
            width: 1000,
            height: 1000,
        }
    }

    fn new_full() -> Board {
        Board {
            map: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24],
            width: 5,
            height: 5,
        }
    }

    fn new_test() -> Board {
        Board {
            map: vec![0; 25],
            width: 5,
            height: 5,
        }
    }

    pub fn new_example() -> Board {
        Board {
            map: vec![0; 100],
            width: 10,
            height: 10,
        }
    }
    
    pub fn line_increment(&mut self, line: Vec<(usize, usize)>) {
        for (x, y) in line {
            self[(x,y)] += 1;
        }
    }

    pub fn total_intersections(&self) -> usize {
        self.map.iter().filter(|&n| *n >= 2).count()
    }

}

impl std::ops::Index<Pos> for Board {
    type Output = usize;
    fn index(&self, (x, y): Pos) -> &usize {
        &self.map[(y * self.width + x) as usize]
    }
}

impl std::ops::IndexMut<Pos> for Board {
    fn index_mut(&mut self, (x, y): Pos) -> &mut Self::Output {
        &mut self.map[(y * self.width + x) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn index_test() {
        let board: Board = Board::new_full();
        assert_eq!(board[(0,1)], 5);
        assert_eq!(board[(2,3)], 17);
    }

    #[test]
    fn index_access_test() {
        let mut board: Board = Board::new_full();

        board[(2,3)] = 99;

        assert_eq!(board[(2,3)], 99);
    }

    #[test]
    fn line_increment_test() {
        let mut board: Board = Board::new_test();
        let line_input: Vec<(usize, usize)> = vec![(0,0),(1,1),(2,2),(3,3),(4,4),];

        board.line_increment(line_input);

        assert_eq!(board[(0,1)], 0);
        assert_eq!(board[(0,0)], 1);
        assert_eq!(board[(1,1)], 1);
        assert_eq!(board[(2,2)], 1);
        assert_eq!(board[(3,3)], 1);
        assert_eq!(board[(4,4)], 1);
    }

    #[test]
    fn line_intersection_test() {
        let mut board: Board = Board::new_test();
        let line_input: Vec<(usize, usize)> = vec![(0,0),(1,1),(2,2),(3,3),(4,4),];
        board.line_increment(line_input);
        let line_input: Vec<(usize, usize)> = vec![(0,0),(1,1),(2,2),(3,3),(4,4),];
        board.line_increment(line_input);

        let result: usize = board.total_intersections();

        assert_eq!(result, 5);
    }
}

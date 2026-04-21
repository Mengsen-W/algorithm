use std::collections::HashMap;

const DIRS: [&[(i32, i32)]; 2] = [
    &[(-1, 0), (1, 0), (0, -1), (0, 1)],
    &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
];

struct NeighborSum {
    grid: Vec<Vec<i32>>,
    pos: HashMap<i32, (usize, usize)>,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut pos = HashMap::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                pos.insert(val, (i, j));
            }
        }
        NeighborSum { grid, pos }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.get_sum(value, 0)
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.get_sum(value, 1)
    }

    fn get_sum(&self, value: i32, idx: usize) -> i32 {
        if let Some(&(x, y)) = self.pos.get(&value) {
            let mut sum = 0;
            for &(dx, dy) in DIRS[idx] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0
                    && (nx as usize) < self.grid.len()
                    && ny >= 0
                    && (ny as usize) < self.grid[0].len()
                {
                    sum += self.grid[nx as usize][ny as usize];
                }
            }
            sum
        } else {
            0
        }
    }
}

fn main() {
    {
        let n = NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        assert_eq!(n.adjacent_sum(1), 6);
        assert_eq!(n.adjacent_sum(4), 16);
        assert_eq!(n.diagonal_sum(4), 16);
        assert_eq!(n.diagonal_sum(8), 4);
    }

    {
        let n = NeighborSum::new(vec![
            vec![1, 2, 0, 3],
            vec![4, 7, 15, 6],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 5],
        ]);
        assert_eq!(n.adjacent_sum(15), 23);
        assert_eq!(n.diagonal_sum(9), 45);
    }
}

struct Solution;

impl Solution {
    pub fn check(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
        let mut count = 0;
        for i in 0..2 {
            for j in 0..2 {
                if grid[(x + i) as usize][(y + j) as usize] == 'B' {
                    count += 1;
                }
            }
        }
        count != 2
    }
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if Self::check(&grid, i, j) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec!['B', 'W', 'B'],
                vec!['B', 'W', 'W'],
                vec!['B', 'W', 'B'],
            ],
            true,
        ),
        (
            vec![
                vec!['B', 'W', 'B'],
                vec!['W', 'B', 'W'],
                vec!['B', 'W', 'B'],
            ],
            false,
        ),
        (
            vec![
                vec!['B', 'W', 'B'],
                vec!['B', 'W', 'W'],
                vec!['B', 'W', 'W'],
            ],
            true,
        ),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::can_make_square(grid), ans);
    }
}

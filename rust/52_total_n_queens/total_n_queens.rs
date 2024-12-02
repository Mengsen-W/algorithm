struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn solve(
            row: usize,
            columns: usize,
            diagonals1: usize,
            diagonals2: usize,
            n: usize,
        ) -> i32 {
            if row == n {
                return 1;
            } else {
                let mut count = 0;
                let mut available_positions = ((1 << n) - 1) & !(columns | diagonals1 | diagonals2);
                while available_positions != 0 {
                    let position = available_positions & available_positions.wrapping_neg();
                    available_positions &= available_positions - 1;
                    count += solve(
                        row + 1,
                        columns | position,
                        (diagonals1 | position) << 1,
                        (diagonals2 | position) >> 1,
                        n,
                    );
                }
                return count;
            }
        }

        solve(0, 0, 0, 0, n as usize)
    }
}

fn main() {
    let tests = vec![(4, 2), (1, 1)];

    for (n, ans) in tests {
        assert_eq!(Solution::total_n_queens(n), ans);
    }
}

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![1; (row_index + 1) as usize];
        for i in 1..=row_index {
            row[i as usize] =
                (row[i as usize - 1] as i64 * (row_index - i + 1) as i64 / i as i64) as i32;
        }
        row
    }
}

fn main() {
    let tests = vec![(3, vec![1, 3, 3, 1]), (0, vec![1]), (1, vec![1, 1])];

    for (row_index, ans) in tests {
        assert_eq!(Solution::get_row(row_index), ans);
    }
}

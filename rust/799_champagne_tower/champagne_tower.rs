struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut row = vec![poured as f64];
        for i in 1..=query_row {
            let mut next_row = vec![0.0; i as usize + 1];
            for j in 0..row.len() {
                let volume = row[j];
                if volume > 1.0 {
                    next_row[j] += (volume - 1.0) / 2.0;
                    next_row[j + 1] += (volume - 1.0) / 2.0;
                }
            }
            row = next_row;
        }
        (row[query_glass as usize]).min(1.0)
    }
}

fn main() {
    let tests = vec![(1, 1, 1, 0.0), (2, 1, 1, 0.5), (100000009, 33, 17, 1.0)];

    for (poured, query_row, query_glass, ans) in tests {
        assert_eq!(
            Solution::champagne_tower(poured, query_row, query_glass),
            ans
        );
    }
}

/*
 * @Date: 2024-03-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-27
 * @FilePath: /algorithm/rust/2580_count_ways/count_ways.rs
 */

struct Solution;

impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let mut ranges = ranges;
        const MOD: i64 = 1_000_000_007;
        ranges.sort_by(|a, b| a[0].cmp(&b[0]));

        let n = ranges.len();
        let mut res: i64 = 1;
        let mut i = 0;
        while i < n {
            let mut r = ranges[i][1];
            let mut j = i + 1;
            while j < n && ranges[j][0] <= r {
                r = r.max(ranges[j][1]);
                j += 1;
            }
            res = (res * 2) % MOD;
            i = j;
        }
        res as i32
    }
}

fn main() {
    let tests = vec![
        (vec![vec![6, 10], vec![5, 15]], 2),
        (vec![vec![1, 3], vec![10, 20], vec![2, 5], vec![4, 8]], 4),
    ];

    for (ranges, ans) in tests {
        assert_eq!(Solution::count_ways(ranges), ans);
    }
}

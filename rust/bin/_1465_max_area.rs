/*
 * @Date: 2023-10-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-27
 * @FilePath: /algorithm/rust/1465_max_area/max_area.rs
 */

struct Solution;

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let max_h = Self::get_max_size(h, horizontal_cuts);
        let max_w = Self::get_max_size(w, vertical_cuts);
        (max_h as i64 * max_w as i64 % 1_000_000_007) as i32
    }

    fn get_max_size(size: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.sort_unstable();
        let mut res = cuts[0].max(size - cuts[cuts.len() - 1]);
        for i in 1..cuts.len() {
            res = res.max(cuts[i] - cuts[i - 1]);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (5, 4, vec![1, 2, 4], vec![1, 3], 4),
        (5, 4, vec![3, 1], vec![1], 6),
    ];

    for (h, w, horizontal_cuts, vertical_cuts, ans) in tests {
        assert_eq!(
            Solution::max_area(h, w, horizontal_cuts, vertical_cuts),
            ans
        );
    }
}

/*
 * @Date: 2023-08-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-27
 * @FilePath: /algorithm/rust/56_merge/merge.rs
 */

struct Solution;
impl Solution {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut arr: Vec<Vec<i32>> = [].to_vec();
        let mut intervals = intervals;
        intervals.sort();
        for value in intervals.iter() {
            let left = value[0];
            let right = value[1];
            if arr.is_empty() {
                arr.push([left, right].to_vec());
            } else if let Some(last) = arr.last_mut() {
                if last[1] < left {
                    arr.push([left, right].to_vec());
                } else {
                    last[1] = right.max(last[1]);
                }
            }
        }
        return arr;
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        ),
        (vec![vec![1, 4], vec![4, 5]], vec![vec![1, 5]]),
        (vec![vec![1, 4], vec![0, 4]], vec![vec![0, 4]]),
    ];

    for (intervals, ans) in tests {
        assert_eq!(Solution::merge(intervals), ans);
    }
}

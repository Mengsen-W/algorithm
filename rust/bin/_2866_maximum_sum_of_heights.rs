/*
 * @Date: 2023-12-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-22
 * @FilePath: /algorithm/rust/2866_maximum_sum_of_heights/maximum_sum_of_heights.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut res = 0;
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();

        for i in 0..n {
            while !stack1.is_empty() && max_heights[i] < max_heights[*stack1.last().unwrap()] {
                stack1.pop();
            }
            if stack1.is_empty() {
                prefix[i] = (i as i64 + 1) * max_heights[i] as i64;
            } else {
                prefix[i] = prefix[*stack1.last().unwrap()]
                    + ((i - stack1.last().unwrap()) as i64) * max_heights[i] as i64;
            }
            stack1.push(i);
        }
        for i in (0..n).rev() {
            while !stack2.is_empty() && max_heights[i] < max_heights[*stack2.last().unwrap()] {
                stack2.pop();
            }
            if stack2.is_empty() {
                suffix[i] = ((n - i) as i64) * max_heights[i] as i64;
            } else {
                suffix[i] = suffix[*stack2.last().unwrap()]
                    + ((stack2.last().unwrap() - i) as i64) * max_heights[i] as i64;
            }
            stack2.push(i);
            res = res.max(prefix[i] + suffix[i] - max_heights[i] as i64);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![5, 3, 4, 1, 1], 13),
        (vec![6, 5, 3, 9, 2, 7], 22),
        (vec![3, 2, 5, 5, 2, 3], 18),
    ];

    for (max_heights, ans) in tests {
        assert_eq!(Solution::maximum_sum_of_heights(max_heights), ans);
    }
}

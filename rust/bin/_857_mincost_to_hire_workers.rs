/*
 * @Date: 2022-09-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-02
 * @FilePath: /algorithm/rust/857_mincost_to_hire_workers/mincost_to_hire_workers.rs
 */

struct Solution;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let n = quality.len();
        let mut workers = (0..n).collect::<Vec<_>>();
        let rate = (0..n)
            .map(|i| wage[i] as f64 / quality[i] as f64)
            .collect::<Vec<_>>();
        workers.sort_by(|&a, &b| rate[a].partial_cmp(&rate[b]).unwrap());
        let mut queue = std::collections::BinaryHeap::new();
        let mut sum = 0;

        workers.into_iter().fold(f64::MAX, |mut prev, i| {
            queue.push(quality[i]);
            sum += quality[i];

            if queue.len() > k as usize {
                sum -= queue.pop().unwrap();
            }

            if queue.len() == k as usize {
                prev = prev.min(rate[i] * sum as f64);
            }

            prev
        })
    }
}

fn main() {
    let tests = vec![
        (vec![10, 20, 5], vec![70, 50, 30], 2, 105.00000),
        (
            vec![3, 1, 10, 10, 1],
            vec![4, 8, 2, 2, 7],
            3,
            30.666666666666664,
        ),
    ];

    for (quality, wage, k, ans) in tests {
        assert_eq!(Solution::mincost_to_hire_workers(quality, wage, k), ans);
    }
}

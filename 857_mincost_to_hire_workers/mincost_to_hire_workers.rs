/*
 * @Date: 2022-09-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-11
 * @FilePath: /algorithm/857_mincost_to_hire_workers/mincost_to_hire_workers.rs
 */

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

fn main() {
    {
        let quality = vec![10, 20, 5];
        let wage = vec![70, 50, 30];
        let k = 2;
        let ans = 105.00000;
        assert_eq!(mincost_to_hire_workers(quality, wage, k), ans);
    }

    {
        let quality = vec![3, 1, 10, 10, 1];
        let wage = vec![4, 8, 2, 2, 7];
        let k = 3;
        let ans = 30.666666666666664;
        assert_eq!(mincost_to_hire_workers(quality, wage, k), ans);
    }
}

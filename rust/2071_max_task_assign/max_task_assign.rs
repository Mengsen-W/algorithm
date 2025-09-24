struct Solution;

impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        use std::cmp::{max, min};
        use std::collections::VecDeque;
        let n = tasks.len();
        let m = workers.len();
        let mut tasks = tasks;
        let mut workers = workers;
        tasks.sort();
        workers.sort();

        let check = |mid: usize| -> bool {
            let mut p = pills;
            let mut ws = VecDeque::new();
            let mut ptr = m - 1;
            // 从大到小枚举每一个任务
            for i in (0..mid).rev() {
                while ptr as i32 >= (m - mid) as i32 && workers[ptr] + strength >= tasks[i] {
                    ws.push_front(workers[ptr]);
                    ptr -= 1;
                }
                if ws.is_empty() {
                    return false;
                }
                // 如果双端队列中最大的元素大于等于 tasks[i]
                if *ws.back().unwrap() >= tasks[i] {
                    ws.pop_back();
                } else {
                    if p == 0 {
                        return false;
                    }
                    p -= 1;
                    ws.pop_front();
                }
            }
            true
        };

        let mut left = 1;
        let mut right = min(m, n);
        let mut ans = 0;
        while left <= right {
            let mid = (left + right) / 2;
            if check(mid) {
                ans = mid as i32;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![3, 2, 1], vec![0, 3, 3], 1, 1, 3),
        (vec![5, 4], vec![0, 0, 0], 1, 5, 1),
        (vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10, 2),
        (vec![5, 9, 8, 5, 9], vec![1, 6, 4, 2, 6], 1, 5, 3),
    ];

    for (tasks, workers, pills, strength, ans) in tests {
        assert_eq!(
            Solution::max_task_assign(tasks, workers, pills, strength),
            ans
        );
    }
}

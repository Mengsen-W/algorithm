/*
 * @Date: 2023-05-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-05
 * @FilePath: /algorithm/rust/2432_hardest_worker/hardest_worker.rs
 */

pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
    logs.iter()
        .fold((logs[0][0], 0, 0), |(id, prev, max), v| {
            if v[1] - prev > max {
                (v[0], v[1], v[1] - prev)
            } else if v[1] - prev == max {
                (id.min(v[0]), v[1], v[1] - prev)
            } else {
                (id, v[1], max)
            }
        })
        .0
}

fn main() {
    {
        let n = 10;
        let logs = vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]];
        let ans = 1;
        assert_eq!(hardest_worker(n, logs), ans);
    }

    {
        let n = 26;
        let logs = vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]];
        let ans = 3;
        assert_eq!(hardest_worker(n, logs), ans);
    }

    {
        let n = 10;
        let logs = vec![vec![0, 10], vec![1, 20]];
        let ans = 0;
        assert_eq!(hardest_worker(n, logs), ans);
    }
}

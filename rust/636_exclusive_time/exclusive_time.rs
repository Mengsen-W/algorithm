/*
 * @Date: 2022-08-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-07
 * @FilePath: /algorithm/636_exclusive_time/exclusive_time.rs
 */

pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
    let mut ans = vec![0; n as usize];
    let mut stack = Vec::with_capacity(logs.len() >> 1);
    for log in logs {
        let log: Vec<_> = log.split(':').collect();
        let idx: usize = log[0].parse().unwrap();
        let time: i32 = log[2].parse().unwrap();
        if log[1] == "start" {
            stack.push((idx, time, 0));
        } else {
            let (idx, start, other) = stack.pop().unwrap();
            ans[idx] += time - start - other + 1;
            if let Some(t) = stack.last_mut() {
                t.2 += time - start + 1;
            }
        }
    }
    ans
}

fn main() {
    {
        let n = 2;
        let logs = vec!["0:start:0", "1:start:2", "1:end:5", "0:end:6"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = vec![3, 4];
        assert_eq!(exclusive_time(n, logs), ans);
    }
    {
        let n = 1;
        let logs = vec![
            "0:start:0",
            "0:start:2",
            "0:end:5",
            "0:start:6",
            "0:end:6",
            "0:end:7",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans = vec![8];
        assert_eq!(exclusive_time(n, logs), ans)
    }
    {
        let n = 2;
        let logs = vec![
            "0:start:0",
            "0:start:2",
            "0:end:5",
            "1:start:6",
            "1:end:6",
            "0:end:7",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans = vec![7, 1];
        assert_eq!(exclusive_time(n, logs), ans)
    }
    {
        let n = 2;
        let logs = vec![
            "0:start:0",
            "0:start:2",
            "0:end:5",
            "1:start:7",
            "1:end:7",
            "0:end:8",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans = vec![8, 1];
        assert_eq!(exclusive_time(n, logs), ans);
    }
    {
        let n = 1;
        let logs = vec!["0:start:0", "0:end:0"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = vec![1];
        assert_eq!(exclusive_time(n, logs), ans);
    }
}

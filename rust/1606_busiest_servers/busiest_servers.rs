struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;
        let mut available: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        for i in 0..k {
            available.push(Reverse(i));
        }

        let mut busy: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        let mut requests = vec![0; k];

        for (i, (&start, &t)) in arrival.iter().zip(load.iter()).enumerate() {
            while let Some(&Reverse((finish, id))) = busy.peek() {
                if finish > start {
                    break;
                }
                busy.pop();
                // 保证得到的是一个不小于 i 的且与 id 同余的数
                let i32_i = i as i32;
                let i32_id = id as i32;
                let i32_k = k as i32;
                let pos = (i32_i + ((i32_id - i32_i) % i32_k + i32_k) % i32_k) as usize;
                available.push(Reverse(pos));
            }

            if available.is_empty() {
                continue;
            }

            let Reverse(pos) = available.pop().unwrap();
            let server_id = pos % k;
            requests[server_id] += 1;
            busy.push(Reverse((start + t, server_id)));
        }

        let max_request = *requests.iter().max().unwrap();
        requests
            .iter()
            .enumerate()
            .filter(|&(_, &req)| req == max_request)
            .map(|(i, _)| i as i32)
            .collect()
    }
}

fn main() {
    let tests = vec![
        (3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3], vec![1]),
        (3, vec![1, 2, 3, 4], vec![1, 2, 1, 2], vec![0]),
        (3, vec![1, 2, 3], vec![10, 12, 11], vec![0, 1, 2]),
        (
            3,
            vec![1, 2, 3, 4, 8, 9, 10],
            vec![5, 2, 10, 3, 1, 2, 2],
            vec![1],
        ),
        (1, vec![1], vec![1], vec![0]),
    ];

    for (i, (k, arrival, load, ans)) in tests.iter().enumerate() {
        assert_eq!(
            Solution::busiest_servers(*k, arrival.clone(), load.clone()),
            *ans,
            "{}",
            i
        );
    }
}

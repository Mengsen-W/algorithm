/*
 * @Date: 2022-01-22 08:47:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-22 09:12:40
 */

struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        if len_n == 1 {
            return 0;
        }
        fn build_graph(nums: &Vec<i32>) -> HashMap<i32, Vec<usize>> {
            let mut graph: HashMap<i32, Vec<usize>> = HashMap::new();
            for (idx, &num) in nums.iter().enumerate() {
                graph.entry(num).or_default().push(idx);
            }
            graph
        }
        let mut graph = build_graph(&nums);
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(0);
        let mut steps: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if cur == len_n - 1 {
                        return steps;
                    }
                    if let Some(nxts) = graph.get_mut(&nums[cur]) {
                        if cur >= 1 {
                            nxts.push(cur - 1);
                        }
                        if cur + 1 < len_n {
                            nxts.push(cur + 1);
                        }
                        while let Some(nxt) = nxts.pop() {
                            if seen.insert(nxt) {
                                queue.push_back(nxt);
                            }
                        }
                    };
                }
            }
            steps += 1;
        }
        steps
    }
}

fn main() {
    assert_eq!(
        Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
        3
    );
    assert_eq!(Solution::min_jumps(vec![7]), 0);
    assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    assert_eq!(Solution::min_jumps(vec![6, 1, 9]), 2);
    assert_eq!(
        Solution::min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13]),
        3
    );
}

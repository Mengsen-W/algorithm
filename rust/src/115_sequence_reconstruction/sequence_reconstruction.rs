/*
 * @Date: 2022-07-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-23
 * @FilePath: /algorithm/115_sequence_reconstruction/sequence_reconstruction.rs
 */

pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
    use std::collections::HashSet;
    use std::collections::VecDeque;
    let n = nums.len();
    let mut indegree = vec![0; n];
    let mut vis = vec![false; n];
    let mut graph = vec![HashSet::new(); n];

    for seq in sequences {
        let m = seq.len();

        for i in 0..m {
            let j = seq[i] as usize - 1;

            if j >= n {
                return false;
            }

            vis[j] = true;

            if i + 1 < m {
                let k = seq[i + 1] as usize - 1;

                if k >= n || k == j {
                    return false;
                }

                if graph[j].insert(k) {
                    indegree[k] += 1;
                }
            }
        }
    }

    if vis.iter().any(|&v| v == false) {
        return false;
    }

    let mut queue = VecDeque::new();
    let mut stack = vec![];

    for i in 0..n {
        if indegree[i] == 0 {
            queue.push_back(i);
        }
    }

    while let Some(i) = queue.pop_front() {
        if queue.len() > 0 {
            return false;
        }

        stack.push(i as i32 + 1);

        for &j in graph[i].iter() {
            indegree[j] -= 1;

            if indegree[j] == 0 {
                queue.push_back(j);
            }
        }
    }

    stack == nums
}

fn main() {
    {
        let nums = vec![1, 2, 3];
        let sequences = vec![vec![1, 2], vec![1, 3]];
        let ans = false;
        assert_eq!(sequence_reconstruction(nums, sequences), ans);
    }

    {
        let nums = vec![1, 2, 3];
        let sequences = vec![vec![1, 2]];
        let ans = false;
        assert_eq!(sequence_reconstruction(nums, sequences), ans)
    }

    {
        let nums = vec![1, 2, 3];
        let sequences = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let ans = true;
        assert_eq!(sequence_reconstruction(nums, sequences), ans);
    }
}

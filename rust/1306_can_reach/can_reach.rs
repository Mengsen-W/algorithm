struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        use std::collections::VecDeque;
        let start = start as usize;
        if arr[start] == 0 {
            return true;
        }

        let n = arr.len();
        let mut used = vec![false; n];
        used[start] = true;
        let mut q = VecDeque::new();
        q.push_back(start);

        while let Some(u) = q.pop_front() {
            for &v in &[u as i32 + arr[u], u as i32 - arr[u]] {
                if 0 <= v && (v as usize) < n {
                    let v = v as usize;
                    if !used[v] {
                        if arr[v] == 0 {
                            return true;
                        }
                        q.push_back(v);
                        used[v] = true;
                    }
                }
            }
        }

        false
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 3, 0, 3, 1, 2], 5, true),
        (vec![4, 2, 3, 0, 3, 1, 2], 0, true),
        (vec![3, 0, 2, 1, 2], 2, false),
    ];

    for (arr, start, expected) in tests {
        assert_eq!(Solution::can_reach(arr, start), expected);
    }
}

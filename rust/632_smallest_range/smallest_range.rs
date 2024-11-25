struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        let size = nums.len();
        let mut indices: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;

        for i in 0..size {
            for &x in &nums[i] {
                indices.entry(x).or_insert_with(Vec::new).push(i);
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
        }

        let mut freq = vec![0; size];
        let mut inside = 0;
        let mut left = x_min;
        let mut right = x_min - 1;
        let mut best_left = x_min;
        let mut best_right = x_max;

        while right < x_max {
            right += 1;
            if let Some(vec) = indices.get(&right) {
                for &x in vec {
                    freq[x] += 1;
                    if freq[x] == 1 {
                        inside += 1;
                    }
                }
                while inside == size {
                    if right - left < best_right - best_left {
                        best_left = left;
                        best_right = right;
                    }
                    if let Some(vec) = indices.get(&left) {
                        for &x in vec {
                            freq[x] -= 1;
                            if freq[x] == 0 {
                                inside -= 1;
                            }
                        }
                    }
                    left += 1;
                }
            }
        }

        vec![best_left, best_right]
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![4, 10, 15, 24, 26],
                vec![0, 9, 12, 20],
                vec![5, 18, 22, 30],
            ],
            vec![20, 24],
        ),
        (
            vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]],
            vec![1, 1],
        ),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::smallest_range(nums), ans);
    }
}

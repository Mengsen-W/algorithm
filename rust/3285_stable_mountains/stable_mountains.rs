struct Solution;

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 1..height.len() {
            if height[i - 1] > threshold {
                result.push(i as i32);
            }
        }
        result
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5], 2, vec![3, 4]),
        (vec![10, 1, 10, 1, 10], 3, vec![1, 3]),
        (vec![10, 1, 10, 1, 10], 10, vec![]),
    ];

    for (height, threshold, ans) in tests {
        assert_eq!(Solution::stable_mountains(height, threshold), ans);
    }
}

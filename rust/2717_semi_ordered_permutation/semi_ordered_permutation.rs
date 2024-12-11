struct Solution;

impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut first = 0;
        let mut last = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num == 1 {
                first = i as i32;
            }
            if num == n {
                last = i as i32;
            }
        }
        first + n - 1 - last - if last < first { 1 } else { 0 }
    }
}

fn main() {
    let tests = vec![
        (vec![2, 1, 4, 3], 2),
        (vec![2, 4, 1, 3], 3),
        (vec![1, 3, 4, 2, 5], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::semi_ordered_permutation(nums), ans);
    }
}

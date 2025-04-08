struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut seen = [false; 128];
        for i in (0..nums.len()).rev() {
            let num = nums[i] as usize;
            if seen[num] {
                return (i as i32) / 3 + 1;
            }
            seen[num] = true;
        }
        0
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 2, 3, 3, 5, 7], 2),
        (vec![4, 5, 6, 4, 4], 2),
        (vec![6, 7, 8, 9], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::minimum_operations(nums), ans);
    }
}

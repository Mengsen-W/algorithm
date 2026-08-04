struct Solution;

impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut ans = Vec::new();
        for i in 0..nums.len() - 1 {
            for x in nums[i] + 1..nums[i + 1] {
                ans.push(x);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 4, 2, 5], vec![3]),
        (vec![7, 8, 6, 9], vec![]),
        (vec![5, 1], vec![2, 3, 4]),
    ];

    for (test, expected) in tests {
        assert_eq!(Solution::find_missing_elements(test), expected);
    }
}

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_pos = 0;
        let mut end = 0;
        let mut steps = 0;
        for i in 0..nums.len() - 1 {
            if max_pos >= i {
                max_pos = max_pos.max(i + nums[i] as usize);
                if i == end {
                    end = max_pos;
                    steps += 1;
                }
            }
        }
        steps
    }
}

fn main() {
    let tests = vec![(vec![2, 3, 1, 1, 4], 2), (vec![2, 3, 0, 1, 4], 2)];

    for (nums, ans) in tests {
        assert_eq!(Solution::jump(nums), ans);
    }
}

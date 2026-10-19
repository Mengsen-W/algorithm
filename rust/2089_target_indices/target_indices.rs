struct Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cnt1 = 0; // 小于 target 的元素数量
        let mut cnt2 = 0; // 等于 target 的元素数量

        for &num in nums.iter() {
            if num < target {
                cnt1 += 1;
            } else if num == target {
                cnt2 += 1;
            }
        }

        let mut res = Vec::with_capacity(cnt2);
        for i in cnt1..cnt1 + cnt2 {
            res.push(i as i32);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 5, 2, 3], 2, vec![1, 2]),
        (vec![1, 2, 5, 2, 3], 3, vec![3]),
        (vec![1, 2, 5, 2, 3], 5, vec![4]),
        (vec![1, 2, 5, 2, 3], 4, vec![]),
    ];

    for (nums, target, expected) in tests {
        assert_eq!(Solution::target_indices(nums, target), expected);
    }
}

struct Solution;

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut cnt = 0;
        let mut prev = i32::MIN;

        for num in nums {
            let curr = (num - k).max(prev + 1).min(num + k);
            if curr > prev {
                cnt += 1;
                prev = curr;
            }
        }
        cnt
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 2, 3, 3, 4], 2, 6), (vec![4, 4, 4, 4], 1, 3)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::max_distinct_elements(nums, k), ans);
    }
}

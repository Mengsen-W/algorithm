/*
 * @Date: 2023-08-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-26
 * @FilePath: /algorithm/rust/228_summary_ranges/summary_ranges.rs
 */

struct Solution;
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans = vec![];
        let mut left = 0;
        while left < nums.len() {
            let mut right = left;
            let mut gap = 0;
            while right < nums.len() && nums[right] == nums[left] + gap {
                right += 1;
                gap += 1;
            }
            right -= 1;
            ans.push(if left == right {
                format!("{}", nums[left])
            } else {
                format!("{}->{}", nums[left], nums[right])
            });
            left = right;
            left += 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5], vec!["1->5"]),
        (vec![0, 1, 2, 4, 5, 7], vec!["0->2", "4->5", "7"]),
        (vec![0, 2, 3, 4, 6, 8, 9], vec!["0", "2->4", "6", "8->9"]),
    ];

    for (nums, ans) in tests {
        assert_eq!(
            Solution::summary_ranges(nums),
            ans.iter().map(|s| s.to_string()).collect::<Vec<String>>()
        );
    }
}

/*
 * @Date: 2021-03-24 08:15:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-24 08:24:47
 * @FilePath: \algorithm\456_132_pattern\132_pattern.rs
 * @Description: file content
 */

fn find132_pattern(nums: Vec<i32>) -> bool {
    let mut two = std::i32::MIN;
    let mut s = Vec::new();
    for i in (0..nums.len()).rev() {
        if nums[i] >= two {
            while let Some(x) = s.pop() {
                if nums[x] < nums[i] {
                    two = nums[x]
                } else {
                    s.push(x);
                    break;
                }
            }
            s.push(i);
        } else {
            return true;
        }
    }
    false
}

fn main() {
    assert!(!find132_pattern(vec![1, 2, 3, 4]));
    assert!(find132_pattern(vec![3, 1, 4, 2]));
    assert!(find132_pattern(vec![-1, 3, 2, 0]));
}

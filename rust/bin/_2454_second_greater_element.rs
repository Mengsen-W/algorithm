/*
 * @Date: 2023-12-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-12
 * @FilePath: /algorithm/rust/2454_second_greater_element/second_greater_element.rs
 */

struct Solution;

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums.len()];
        let mut s = Vec::new();
        let mut t = Vec::new();
        for (i, &x) in nums.iter().enumerate() {
            while !t.is_empty() && nums[*t.last().unwrap()] < x {
                ans[t.pop().unwrap()] = x; // t 栈顶的下下个更大元素是 x
            }
            let mut j = s.len();
            while j > 0 && nums[s[j - 1]] < x {
                j -= 1; // s 栈顶的下一个更大元素是 x
            }
            t.extend(s.drain(j..)); // 把从 s 弹出的这一整段元素加到 t
            s.push(i); // 当前元素（的下标）加到 s 栈顶
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![2, 4, 0, 9, 6], vec![9, 6, 6, -1, -1]),
        (vec![3, 3], vec![-1, -1]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::second_greater_element(nums), ans);
    }
}

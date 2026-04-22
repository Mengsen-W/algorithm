/*
 * @Date: 2023-07-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-24
 * @FilePath: /algorithm/rust/771_num_jewels_in_stones/num_jewels_in_stones.rs
 */

struct Solution;
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let hs = jewels.chars().collect::<std::collections::HashSet<_>>();
        stones
            .chars()
            .fold(0, |acc, c| if hs.contains(&c) { acc + 1 } else { acc })
    }
}

fn main() {
    let tests = vec![("aA", "aAAbbbb", 3), ("z", "ZZ", 0)];
    for (jewels, stones, ans) in tests {
        assert_eq!(
            Solution::num_jewels_in_stones(jewels.to_string(), stones.to_string()),
            ans
        )
    }
}

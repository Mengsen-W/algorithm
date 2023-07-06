/*
 * @Date: 2023-07-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-06
 * @FilePath: /algorithm/rust/2718_maximum_even_split/maximum_even_split.rs
 */

struct Solution;
impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        if final_sum & 1 == 1 {
            return vec![];
        }
        let mut ans = vec![];
        let mut t = 2;
        while t * 2 < final_sum {
            ans.push(t);
            final_sum -= t;
            t += 2;
        }
        ans.push(final_sum);
        ans
    }
}

fn main() {
    let test_map = vec![(12, vec![2, 4, 6]), (7, vec![]), (28, vec![2, 4, 6, 16])];
    for item in test_map {
        assert_eq!(Solution::maximum_even_split(item.0), item.1)
    }
}

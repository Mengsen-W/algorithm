/*
 * @Date: 2023-09-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-26
 * @FilePath: /algorithm/rust/2582_pass_the_pillow/pass_the_pillow.rs
 */
struct Solution;
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let time = time % ((n - 1) * 2);
        if time < n {
            time + 1
        } else {
            n * 2 - time - 1
        }
    }
}

fn main() {
    let tests = vec![(4, 5, 2), (3, 2, 3)];

    for (n, time, ans) in tests {
        assert_eq!(Solution::pass_the_pillow(n, time), ans);
    }
}

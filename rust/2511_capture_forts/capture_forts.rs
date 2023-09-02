/*
 * @Date: 2023-09-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-02
 * @FilePath: /algorithm/rust/2511_capture_forts/capture_forts.rs
 */

struct Solution;
impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pre = -1;

        for (i, &fort) in forts.iter().enumerate() {
            if fort == 1 || fort == -1 {
                if pre >= 0 && fort != forts[pre as usize] {
                    ans = ans.max(i as i32 - pre - 1);
                }
                pre = i as i32;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 0, 0, -1, 0, 0, 0, 0, 1], 4),
        (vec![0, 0, 1, -1], 0),
    ];

    for (forts, ans) in tests {
        assert_eq!(Solution::capture_forts(forts), ans);
    }
}

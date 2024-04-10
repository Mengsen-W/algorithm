/*
 * @Date: 2024-04-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-10
 * @FilePath: /algorithm/rust/1702_maximum_binary_string/maximum_binary_string.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let n = binary.len();
        let mut s: Vec<char> = binary.chars().collect();
        let mut i = n;
        let mut zeros = 0;
        for j in 0..n {
            if s[j] == '0' {
                if i == n {
                    i = j;
                }
                zeros += 1;
            }
            s[j] = '1'
        }
        if zeros == 0 {
            return binary;
        }
        s[i as usize + zeros - 1] = '0';
        s.iter().collect()
    }
}

fn main() {
    let tests = vec![("000110", "111011"), ("01", "01")];

    for (binary, ans) in tests {
        assert_eq!(Solution::maximum_binary_string(binary.to_string()), ans);
    }
}

struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let s = num.to_string(); // num 十进制表示字符串
        let n = s.len();
        let mut res = 0;
        for i in 0..=n - k as usize {
            // 枚举所有长度为 k 的子串
            let tmp: i32 = s[i..i + k as usize].parse().unwrap();
            if tmp != 0 && num % tmp == 0 {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(240, 2, 2), (430043, 2, 2)];

    for (num, k, ans) in tests {
        assert_eq!(Solution::divisor_substrings(num, k), ans);
    }
}

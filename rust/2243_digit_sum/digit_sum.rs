struct Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut s = s;
        let k = k as usize;
        while s.len() > k {
            let mut tmp = String::new(); // 每次操作结束的字符串
            let n = s.len();
            let chars: Vec<char> = s.chars().collect();
            for i in (0..n).step_by(k) {
                let mut val = 0;
                let end = (i + k).min(n);
                for j in i..end {
                    val += chars[j].to_digit(10).unwrap() as i32;
                }
                tmp.push_str(&val.to_string());
            }
            s = tmp;
        }
        s
    }
}

fn main() {
    let tests = vec![("11111222223", 3, "135"), ("00000000", 3, "000")];

    for (s, k, ans) in tests {
        assert_eq!(Solution::digit_sum(s.to_string(), k), ans);
    }
}

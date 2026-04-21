struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut score = 0;
        let bytes = s.as_bytes(); // 转换为字节数组
        for i in 0..bytes.len() - 1 {
            score += (bytes[i] as i32 - bytes[i + 1] as i32).abs();
        }
        score
    }
}

fn main() {
    let tests = vec![("hello", 13), ("zaz", 50)];

    for (s, ans) in tests {
        assert_eq!(Solution::score_of_string(s.to_string()), ans);
    }
}

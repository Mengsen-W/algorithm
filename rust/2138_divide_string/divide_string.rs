struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut res = Vec::new(); // 分组后的字符串
        let n = s.len();
        let mut curr = 0; // 每个分组的起始下标
                          // 拆分字符串
        while curr < n {
            let end = (curr + k as usize).min(n);
            res.push(s[curr..end].to_string());
            curr += k as usize;
        }
        // 尝试填充最后一组
        if let Some(last) = res.last_mut() {
            if last.len() < k as usize {
                *last += &fill.to_string().repeat(k as usize - last.len());
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        ("abcdefghi", 3, 'x', vec!["abc", "def", "ghi"]),
        ("abcdefghij", 3, 'x', vec!["abc", "def", "ghi", "jxx"]),
    ];

    for (s, k, fill, ans) in tests {
        assert_eq!(
            Solution::divide_string(s.to_string(), k, fill),
            ans.iter().map(|s| s.to_string()).collect::<Vec<String>>()
        );
    }
}

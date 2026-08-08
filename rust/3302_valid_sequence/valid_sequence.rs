struct Solution;

impl Solution {
    pub fn valid_sequence(s: String, t: String) -> Vec<i32> {
        let n = s.len();
        let m = t.len();
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        // suf[i] 表示从 s[i..] 开始向后匹配，t 中还有多少个字符未被匹配
        // 等价于：在 s[i..n-1] 中至多能匹配 t 的后缀长度
        let mut suf = vec![m; n + 1];
        let mut j = m as i32 - 1;
        for i in (0..n).rev() {
            if j >= 0 && s_bytes[i] == t_bytes[j as usize] {
                j -= 1;
            }
            suf[i] = (j + 1) as usize;
        }

        let mut ans = vec![0i32; m];
        let mut changed = false; // 是否已经使用过修改机会
        let mut j = 0usize;
        for i in 0..n {
            if s_bytes[i] == t_bytes[j] || (!changed && suf[i + 1] <= j + 1) {
                if s_bytes[i] != t_bytes[j] {
                    changed = true;
                }
                ans[j] = i as i32;
                j += 1;
                if j == m {
                    return ans;
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sequence() {
        assert_eq!(
            Solution::valid_sequence("vbcca".to_string(), "abc".to_string()),
            vec![0, 1, 2]
        );
        assert_eq!(
            Solution::valid_sequence("bacdc".to_string(), "abc".to_string()),
            vec![1, 2, 4]
        );
        assert_eq!(
            Solution::valid_sequence("aaaaaa".to_string(), "aaabc".to_string()),
            vec![]
        );
        assert_eq!(
            Solution::valid_sequence("abc".to_string(), "ab".to_string()),
            vec![0, 1]
        );
    }
}

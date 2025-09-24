struct Solution;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let n = s.len();
        let bytes = s.as_bytes();
        let max_len = n / k as usize;
        // 通过统计频率确定seq中的候选字符
        let mut char_count = vec![0; 26];
        for &ch in bytes {
            char_count[(ch - b'a') as usize] += 1;
        }
        let mut char_list = Vec::with_capacity(max_len);
        for i in 0..26 {
            for _ in 0..char_count[i] / k {
                char_list.push(b'a' + i as u8);
            }
        }
        // dp 预先计算s中每个位置后面第一次出现每个字符的索引
        let mut next_char_map = vec![vec![n; 26]; n + 1];
        for (i, &ch) in bytes.iter().enumerate().rev() {
            for j in 0..26 {
                next_char_map[i][j] = next_char_map[i + 1][j];
            }
            next_char_map[i][ch as usize - 'a' as usize] = i;
        }
        next_char_map.truncate(n);

        // 深度遍历
        if let Some((_, v)) = Solution::bt(max_len, &mut vec![], 0, &char_list, k, &next_char_map) {
            return String::from_utf8(v).unwrap();
        } else {
            return String::from("");
        }
    }

    fn bt(
        max: usize,
        cur: &mut Vec<u8>,
        mask: i32,
        chars: &Vec<u8>,
        k: i32,
        next_char_map: &Vec<Vec<usize>>,
    ) -> Option<(usize, Vec<u8>)> {
        let mut ret = None;
        if !cur.is_empty() && Solution::has_k_subseq(cur, k, next_char_map) {
            ret = Some((cur.len(), cur.clone()));
        }
        if cur.len() < max {
            for (i, &ch) in chars.iter().enumerate() {
                let m = 1 << i;
                if m & mask != 0 {
                    continue;
                }
                cur.push(ch);
                let x = Solution::bt(max, cur, mask | m, chars, k, next_char_map);
                cur.pop();
                ret = std::cmp::max(ret, x);
            }
        }
        return ret;
    }

    fn has_k_subseq(seq: &Vec<u8>, k: i32, next_char_map: &Vec<Vec<usize>>) -> bool {
        let mut cur = 0;
        for _ in 0..k {
            for &ch in seq {
                if cur == next_char_map.len() {
                    return false;
                }
                cur = next_char_map[cur][ch as usize - 'a' as usize];
                if cur == next_char_map.len() {
                    return false;
                }
                cur += 1;
            }
        }
        return true;
    }
}

fn main() {
    let tests = vec![("letsleetcode", 2, "let"), ("bb", 2, "b"), ("ab", 2, "")];

    for (s, k, expected) in tests {
        assert_eq!(
            Solution::longest_subsequence_repeated_k(s.to_string(), k),
            expected
        );
    }
}

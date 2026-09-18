struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        use std::collections::HashMap;
        // 记录每个字符的第一次和最后一次出现位置
        let bytes = s.as_bytes();
        let mut pos: HashMap<u8, (usize, usize)> = HashMap::new();

        for (i, &ch) in bytes.iter().enumerate() {
            if !pos.contains_key(&ch) {
                pos.insert(ch, (i, i));
            } else {
                pos.get_mut(&ch).unwrap().1 = i;
            }
        }

        // 所有合法的区间
        let mut valid: Vec<(usize, usize)> = Vec::new();

        for &(l_c, r_c) in pos.values() {
            let mut l = l_c;
            let mut r = r_c;
            let mut nl = l as i32;
            let mut nr = l as i32;

            while nl >= l as i32 || nr <= r as i32 {
                let i = if nl >= l as i32 {
                    nl as usize
                } else {
                    nr as usize
                };

                // 当前处理的是字符 s[i]
                let current_char = bytes[i];
                let &(l_t, r_t) = pos.get(&current_char).unwrap();

                // 当前区间左侧还有该字符，需要向左扩展
                if l_t < l {
                    l = l_t;
                }

                // 当前区间右侧还有该字符，需要向右扩展
                if r_t > r {
                    r = r_t;
                }

                // 当前处理的是左指针
                if i as i32 == nl {
                    nl -= 1;
                }

                // 当前处理的是右指针
                if i as i32 == nr {
                    nr += 1;
                }
            }

            valid.push((l, r));
        }

        // 按右端点升序排序
        valid.sort_by(|a, b| a.1.cmp(&b.1));

        // 贪心选择互不重叠的区间
        let mut ans: Vec<String> = Vec::new();
        let mut end: i32 = -1;

        for (left, right) in valid {
            if left as i32 > end {
                // 使用字节切片，避免多次字符访问
                ans.push(String::from_utf8(bytes[left..right + 1].to_vec()).unwrap());
                end = right as i32;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        ("adefaddaccc", vec!["e", "f", "ccc"]),
        ("abbaccd", vec!["d", "bb", "cc"]),
    ];

    for (s, ans) in tests {
        assert_eq!(Solution::max_num_of_substrings(s.to_string()), ans);
    }
}

struct Solution;
impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let m = 1000000007;
        let n = pressed_keys.len();
        let mut dp3 = vec![1, 1, 2, 4]; // 连续按多次 3 个字母按键对应的方案数
        let mut dp4 = vec![1, 1, 2, 4]; // 连续按多次 4 个字母按键对应的方案数
        for i in 4..=n {
            dp3.push((dp3[i - 1] + dp3[i - 2] + dp3[i - 3]) % m);
            dp4.push((dp4[i - 1] + dp4[i - 2] + dp4[i - 3] + dp4[i - 4]) % m);
        }
        let mut res = 1i64; // 总方案数
        let mut cnt = 1; // 当前字符连续出现的次数
        let pressed_keys: Vec<char> = pressed_keys.chars().collect();
        for i in 1..n {
            if pressed_keys[i] == pressed_keys[i - 1] {
                cnt += 1;
            } else {
                // 对按键对应字符数量讨论并更新总方案数
                if pressed_keys[i - 1] == '7' || pressed_keys[i - 1] == '9' {
                    res = (res * dp4[cnt]) % m as i64;
                } else {
                    res = (res * dp3[cnt]) % m as i64;
                }
                cnt = 1;
            }
        }
        // 更新最后一段连续字符子串对应的方案数
        if pressed_keys[n - 1] == '7' || pressed_keys[n - 1] == '9' {
            res = (res * dp4[cnt]) % m as i64;
        } else {
            res = (res * dp3[cnt]) % m as i64;
        }
        res as i32
    }
}

fn main() {
    let tests = vec![
        ("22233", 8),
        ("222222222222222222222222222222222222", 82876089),
    ];

    for (pressed_keys, ans) in tests {
        assert_eq!(Solution::count_texts(pressed_keys.to_string()), ans);
    }
}

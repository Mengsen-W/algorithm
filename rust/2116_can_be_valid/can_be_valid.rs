struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        // let n = s.len();
        let mut mx = 0; // 可以达到的最大分数
        let mut mn = 0; // 可以达到的最小分数 与 最小有效前缀对应分数 的较大值
        for (i, (sc, lc)) in s.chars().zip(locked.chars()).enumerate() {
            if lc == '1' {
                // 此时对应字符无法更改
                let diff = if sc == '(' { 1 } else { -1 };
                mx += diff;
                mn = std::cmp::max(mn + diff, (i as i32 + 1) % 2);
            } else {
                // 此时对应字符可以更改
                mx += 1;
                mn = std::cmp::max(mn - 1, (i as i32 + 1) % 2);
            }
            if mx < mn {
                // 此时该前缀无法变为有效前缀
                return false;
            }
        }
        // 最终确定 s 能否通过变换使得分数为 0（成为有效字符串）
        mn == 0
    }
}

fn main() {
    let tests = vec![
        ("))()))", "010100", true),
        ("()()", "0000", true),
        (")", "0", false),
        ("(((())(((())", "111111010111", true),
    ];

    for (s, locked, expected) in tests {
        assert_eq!(
            Solution::can_be_valid(s.to_string(), locked.to_string()),
            expected
        );
    }
}

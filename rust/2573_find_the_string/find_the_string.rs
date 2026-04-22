struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut word = vec!['\0'; n];
        let mut current = b'a';

        // 依次从 'a' 到 'z' 开始构造字符串
        for i in 0..n {
            if word[i] == '\0' {
                if current > b'z' {
                    return String::new();
                }
                word[i] = current as char;
                for j in i + 1..n {
                    if lcp[i][j] > 0 {
                        word[j] = word[i];
                    }
                }
                current += 1;
            }
        }

        // 验证构造的字符串是否满足 LCP 矩阵要求
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if word[i] != word[j] {
                    if lcp[i][j] != 0 {
                        return String::new();
                    }
                } else {
                    if i == n - 1 || j == n - 1 {
                        if lcp[i][j] != 1 {
                            return String::new();
                        }
                    } else {
                        if lcp[i][j] != lcp[i + 1][j + 1] + 1 {
                            return String::new();
                        }
                    }
                }
            }
        }

        word.iter().collect()
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![4, 0, 2, 0],
                vec![0, 3, 0, 1],
                vec![2, 0, 2, 0],
                vec![0, 1, 0, 1],
            ],
            "abab",
        ),
        (
            vec![
                vec![4, 3, 2, 1],
                vec![3, 3, 2, 1],
                vec![2, 2, 2, 1],
                vec![1, 1, 1, 1],
            ],
            "aaaa",
        ),
        (
            vec![
                vec![4, 3, 2, 1],
                vec![3, 3, 2, 1],
                vec![2, 2, 2, 1],
                vec![1, 1, 1, 3],
            ],
            "",
        ),
    ];

    for (lsp, ans) in tests {
        assert_eq!(Solution::find_the_string(lsp), ans);
    }
}

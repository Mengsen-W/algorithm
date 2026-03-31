struct Solution;

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let n = str1.len();
        let m = str2.len();
        let str1_chars: Vec<char> = str1.chars().collect();
        let str2_chars: Vec<char> = str2.chars().collect();

        let mut s = vec!['a'; n + m - 1];
        let mut fixed = vec![false; n + m - 1];

        for i in 0..n {
            if str1_chars[i] == 'T' {
                for j in i..i + m {
                    let target_char = str2_chars[j - i];
                    if fixed[j] && s[j] != target_char {
                        return String::new();
                    } else {
                        s[j] = target_char;
                        fixed[j] = true;
                    }
                }
            }
        }

        for i in 0..n {
            if str1_chars[i] == 'F' {
                let mut flag = false;
                let mut idx = -1;

                for j in (i..i + m).rev() {
                    let target_char = str2_chars[j - i];
                    if target_char != s[j] {
                        flag = true;
                    }
                    if idx == -1 && !fixed[j] {
                        idx = j as i32;
                    }
                }

                if flag {
                    continue;
                } else if idx != -1 {
                    s[idx as usize] = 'b';
                } else {
                    return String::new();
                }
            }
        }

        s.into_iter().collect()
    }
}

fn main() {
    let tests = vec![
        ("TFTF", "ab", "ababa"),
        ("TFTF", "abc", ""),
        ("F", "d", "a"),
    ];

    for (str1, str2, expected) in tests {
        assert_eq!(
            Solution::generate_string(str1.to_string(), str2.to_string()),
            expected.to_string()
        );
    }
}

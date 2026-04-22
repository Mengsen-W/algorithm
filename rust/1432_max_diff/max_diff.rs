struct Solution;

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        // 替换字符串中的字符
        fn replace(s: &str, x: char, y: char) -> String {
            s.chars().map(|c| if c == x { y } else { c }).collect()
        }
        let mut min_num = num.to_string();
        let mut max_num = num.to_string();
        // 找到一个高位替换成 9
        for digit in max_num.chars() {
            if digit != '9' {
                max_num = replace(&max_num, digit, '9');
                break;
            }
        }
        // 将最高位替换成 1
        // 或者找到一个与最高位不相等的高位替换成 0
        for (i, digit) in min_num.chars().enumerate() {
            if i == 0 {
                if digit != '1' {
                    min_num = replace(&min_num, digit, '1');
                    break;
                }
            } else {
                if digit != '0' && digit != min_num.chars().nth(0).unwrap() {
                    min_num = replace(&min_num, digit, '0');
                    break;
                }
            }
        }

        max_num.parse::<i32>().unwrap() - min_num.parse::<i32>().unwrap()
    }
}

fn main() {
    let tests = vec![
        (555, 888),
        (9, 8),
        (123456, 820000),
        (10000, 80000),
        (9288, 8700),
    ];

    for (num, ans) in tests {
        assert_eq!(Solution::max_diff(num), ans);
    }
}

struct Solution;

impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let n = num.len();
        let mut find0 = false;
        let mut find5 = false;
        let num_chars: Vec<char> = num.chars().collect();

        for i in (0..n).rev() {
            if num_chars[i] == '0' || num_chars[i] == '5' {
                if find0 {
                    return (n - i - 2) as i32;
                }
                if num_chars[i] == '0' {
                    find0 = true;
                } else {
                    find5 = true;
                }
            } else if num_chars[i] == '2' || num_chars[i] == '7' {
                if find5 {
                    return (n - i - 2) as i32;
                }
            }
        }

        if find0 {
            return (n - 1) as i32;
        }
        return n as i32;
    }
}

fn main() {
    let tests = vec![("2245047", 2), ("2908305", 3), ("10", 1)];

    for (num, ans) in tests {
        assert_eq!(Solution::minimum_operations(num.to_string()), ans);
    }
}

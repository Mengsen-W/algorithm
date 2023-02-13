/*
 * @Date: 2023-02-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-13
 * @FilePath: /algorithm/rust/1234_balanced_string/balanced_string.rs
 */

pub fn balanced_string(s: String) -> i32 {
    let s = s.as_bytes();
    let mut count = [0; 4];
    let mut l = 0;
    let mut ret = s.len();

    for c in s {
        match c {
            b'Q' => count[0] += 1,
            b'W' => count[1] += 1,
            b'E' => count[2] += 1,
            _ => count[3] += 1,
        }
    }

    for r in 0..=s.len() {
        while l <= r
            && 4 * count.iter().max().unwrap() - count.iter().sum::<i32>() <= (r - l) as i32
        {
            ret = ret.min(r - l);
            match s[l.min(s.len() - 1)] {
                b'Q' => count[0] += 1,
                b'W' => count[1] += 1,
                b'E' => count[2] += 1,
                _ => count[3] += 1,
            }
            l += 1;
        }
        match s[r.min(s.len() - 1)] {
            b'Q' => count[0] -= 1,
            b'W' => count[1] -= 1,
            b'E' => count[2] -= 1,
            _ => count[3] -= 1,
        }
    }

    ret as i32
}

fn main() {
    {
        let s = String::from("QWER");
        let ans = 0;
        assert_eq!(balanced_string(s), ans);
    }

    {
        let s = String::from("QQWE");
        let ans = 1;
        assert_eq!(balanced_string(s), ans);
    }

    {
        let s = String::from("QQQW");
        let ans = 2;
        assert_eq!(balanced_string(s), ans);
    }

    {
        let s = String::from("QQQQ");
        let ans = 3;
        assert_eq!(balanced_string(s), ans);
    }
}

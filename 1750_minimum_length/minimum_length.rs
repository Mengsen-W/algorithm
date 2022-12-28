/*
 * @Date: 2022-12-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-28
 * @FilePath: /algorithm/1750_minimum_length/minimum_length.rs
 */

pub fn minimum_length(s: String) -> i32 {
    let s = s.into_bytes();
    let n = s.len();
    let (mut left, mut right) = (0, n - 1);
    while left < right && s[left] == s[right] {
        let c = s[left];
        while left <= right && s[left] == c {
            left += 1;
        }
        while left <= right && s[right] == c {
            right -= 1;
        }
    }
    (right + 1 - left) as i32
}

fn main() {
    {
        let s = String::from("ca");
        let ans = 2;
        assert_eq!(minimum_length(s), ans);
    }

    {
        let s = String::from("cabaabac");
        let ans = 0;
        assert_eq!(minimum_length(s), ans);
    }

    {
        let s = String::from("aabccabba");
        let ans = 3;
        assert_eq!(minimum_length(s), ans);
    }
}

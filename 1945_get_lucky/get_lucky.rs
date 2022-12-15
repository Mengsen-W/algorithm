/*
 * @Date: 2022-12-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-15
 * @FilePath: /algorithm/1945_get_lucky/get_lucky.rs
 */

pub fn get_lucky(s: String, k: i32) -> i32 {
    (0..k - 1).fold(
        s.bytes()
            .map(|x| (x - 96) as i32)
            .map(|x| x / 10 + x % 10)
            .sum(),
        |x, _| x.to_string().bytes().map(|x| (x - 48) as i32).sum(),
    )
}

fn main() {
    {
        let s = String::from("iiii");
        let k = 1;
        let ans = 36;
        assert_eq!(get_lucky(s, k), ans);
    }

    {
        let s = String::from("leetcode");
        let k = 2;
        let ans = 6;
        assert_eq!(get_lucky(s, k), ans);
    }
}

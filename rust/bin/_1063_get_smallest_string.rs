/*
 * @Date: 2023-01-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-26
 * @FilePath: /algorithm/rust/1063_get_smallest_string/get_smallest_string.rs
 */

pub fn get_smallest_string(n: i32, mut k: i32) -> String {
    (1..=n).fold(String::new(), |mut ans, i| {
        let lower = 1.max(k - (n - i) * 26);
        k -= lower;
        ans.push(char::from_u32((97 + lower - 1) as u32).unwrap());
        ans
    })
}

fn main() {
    {
        let n = 3;
        let k = 27;
        let ans = String::from("aay");
        assert_eq!(get_smallest_string(n, k), ans);
    }

    {
        let n = 5;
        let k = 73;
        let ans = String::from("aaszz");
        assert_eq!(get_smallest_string(n, k), ans);
    }
}

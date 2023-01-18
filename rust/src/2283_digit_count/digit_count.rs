/*
 * @Date: 2023-01-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-11
 * @FilePath: /algorithm/2283_digit_count/digit_count.rs
 */

pub fn digit_count(num: String) -> bool {
    let mut count_map = [0_u8; 10];
    num.bytes()
        .for_each(|c| count_map[(c - b'0') as usize] += 1);
    num.bytes()
        .enumerate()
        .all(|(i, c)| c - b'0' == count_map[i])
}

fn main() {
    {
        let num = String::from("1210");
        let ans = true;
        assert_eq!(digit_count(num), ans);
    }

    {
        let num = String::from("030");
        let ans = false;
        assert_eq!(digit_count(num), ans);
    }
}

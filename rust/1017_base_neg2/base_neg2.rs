/*
 * @Date: 2023-04-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-06
 * @FilePath: /algorithm/rust/1017_base_neg2/base_neg2.rs
 */

pub fn base_neg2(n: i32) -> String {
    let mut val = 0x55555555 ^ (0x55555555 - n) as u32;
    if val == 0 {
        return String::from("0");
    }
    let mut res = Vec::new();
    while val != 0 {
        res.push(char::from_u32('0' as u32 + (val & 1)).unwrap());
        val >>= 1;
    }
    res.reverse();
    res.into_iter().collect()
}

fn main() {
    {
        let n = 2;
        let ans = "110";
        assert_eq!(base_neg2(n), ans);
    }

    {
        let n = 3;
        let ans = "111";
        assert_eq!(base_neg2(n), ans);
    }

    {
        let n = 4;
        let ans = "100";
        assert_eq!(base_neg2(n), ans);
    }
}

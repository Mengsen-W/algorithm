/*
 * @Date: 2022-12-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-26
 * @FilePath: /algorithm/1759_count_homogenous/count_homogenous.rs
 */

pub fn count_homogenous(s: String) -> i32 {
    const M: i64 = 1000000007;
    let s = s.into_bytes();
    let mut prev = s[0];
    let mut cnt: i64 = 0;
    let mut res: i64 = 0;
    for c in s {
        if c == prev {
            cnt += 1;
        } else {
            res += (cnt + 1) * cnt / 2;
            cnt = 1;
            prev = c;
        }
    }
    res += (cnt + 1) * cnt / 2;
    (res % M) as i32
}

fn main() {
    {
        let s = String::from("abbcccaa");
        let ans = 13;
        assert_eq!(count_homogenous(s), ans);
    }

    {
        let s = String::from("xy");
        let ans = 2;
        assert_eq!(count_homogenous(s), ans);
    }

    {
        let s = String::from("zzzzz");
        let ans = 15;
        assert_eq!(count_homogenous(s), ans);
    }
}

/*
 * @Date: 2023-01-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-13
 * @FilePath: /algorithm/2287_rearrange_characters/rearrange_characters.rs
 */

pub fn rearrange_characters(s: String, target: String) -> i32 {
    const A: usize = 'a' as usize;
    let cnt_s = s.bytes().into_iter().fold([0; 26], |mut t, c| {
        t[c as usize - A] += 1;
        t
    });

    let cnt_t = target.bytes().into_iter().fold([0; 26], |mut t, c| {
        t[c as usize - A] += 1;
        t
    });
    let mut ans = s.len();
    for (i, c) in cnt_t.iter().enumerate() {
        if c > &0 {
            ans = ans.min(cnt_s[i] / c);
            if ans == 0 {
                return 0;
            }
        }
    }
    ans as i32
}

fn main() {
    {
        let s = "ilovecodingonleetcode".to_string();
        let target = "code".to_string();
        let ans = 2;
        assert_eq!(rearrange_characters(s, target), ans);
    }

    {
        let s = "abcba".to_string();
        let target = "abc".to_string();
        let ans = 1;
        assert_eq!(rearrange_characters(s, target), ans);
    }

    {
        let s = "abbaccaddaeea".to_string();
        let target = "aaaaa".to_string();
        let ans = 1;
        assert_eq!(rearrange_characters(s, target), ans);
    }
}

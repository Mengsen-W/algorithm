/*
 * @Date: 2021-08-21 14:26:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-21 14:54:43
 */

struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut r = 0;
        let mut w = 0;
        while r < chars.len() {
            let i = r;
            while r < chars.len() && chars[r] == chars[i] {
                r += 1;
            }
            chars[w] = chars[i];
            w += 1;
            if r > i + 1 {
                let count = format!("{}", r - i);
                for c in count.chars() {
                    chars[w] = c;
                    w += 1;
                }
            }
        }
        chars.resize(w, '0');
        w as i32
    }
}

fn main() {
    {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let ans = vec!['a', '2', 'b', '2', 'c', '3'];
        assert_eq!(Solution::compress(&mut chars), 6);
        assert_eq!(chars, ans);
    }
    {
        let mut chars = vec!['a'];
        let ans = vec!['a'];
        assert_eq!(Solution::compress(&mut chars), 1);
        assert_eq!(chars, ans);
    }
    {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let ans = vec!['a', 'b', '1', '2'];
        assert_eq!(Solution::compress(&mut chars), 4);
        assert_eq!(chars, ans);
    }
}

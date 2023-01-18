/*
 * @Date: 2022-03-29 01:59:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-29 02:31:59
 */

pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    fn max_consecutive_char(answer_key: &Vec<char>, k: i32, ch: char) -> i32 {
        let n = answer_key.len();
        let mut ans = 0;
        let (mut left, mut right, mut sum) = (0, 0, 0);
        while right < n {
            sum += if answer_key[right] != ch { 1 } else { 0 };
            while sum > k {
                sum -= if answer_key[left] != ch { 1 } else { 0 };
                left += 1;
            }
            ans = ans.max(right - left + 1);
            right += 1;
        }
        ans as i32
    }

    let answer_key = answer_key.chars().collect::<Vec<char>>();
    max_consecutive_char(&answer_key, k, 'T').max(max_consecutive_char(&answer_key, k, 'F'))
}

fn main() {
    assert_eq!(max_consecutive_answers("TTFF".to_string(), 2), 4);
    assert_eq!(max_consecutive_answers("TFFT".to_string(), 1), 3);
    assert_eq!(max_consecutive_answers("TTFTTFTT".to_string(), 1), 5);
}

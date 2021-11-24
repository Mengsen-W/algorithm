/*
 * @Date: 2021-11-24 00:52:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-24 00:59:15
 * @FilePath: /algorithm/423_original_digits/original_digits.rs
 * @Description: file content
 */

pub fn original_digits(s: String) -> String {
    let mut result = Vec::new();
    let mut nums = [0].repeat(10);
    let mut cnt = [0].repeat(26);
    for c in s.chars() {
        let pos: usize = (c as u8 - b'a').into();
        cnt[pos] = cnt[pos] + 1;
    }
    nums[6] = cnt[(b'x' - b'a') as usize];
    nums[0] = cnt[(b'z' - b'a') as usize];
    nums[8] = cnt[(b'g' - b'a') as usize];
    nums[4] = cnt[(b'u' - b'a') as usize];
    nums[2] = cnt[(b'w' - b'a') as usize];
    nums[7] = cnt[(b's' - b'a') as usize] - nums[6];
    nums[3] = cnt[(b'h' - b'a') as usize] - nums[8];
    nums[1] = cnt[(b'o' - b'a') as usize] - nums[0] - nums[2] - nums[4];
    nums[5] = cnt[(b'v' - b'a') as usize] - nums[7];
    nums[9] = cnt[(b'i' - b'a') as usize] - nums[8] - nums[6] - nums[5];

    for (i, &num) in nums.iter().enumerate() {
        result.append(&mut [i as u8 + b'0'].repeat(num));
    }
    String::from_utf8(result).unwrap()
}

fn main() {
    assert_eq!(original_digits("owoztneoer".to_string()), "012");
    assert_eq!(original_digits("fviefuro".to_string()), "45");
}

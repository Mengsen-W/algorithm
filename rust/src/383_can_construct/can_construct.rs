/*
 * @Date: 2021-12-04 05:44:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-04 05:45:27
 */

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() > magazine.len() {
        return false;
    }
    let mut hash = [0; 26];
    magazine
        .chars()
        .for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] += 1);
    ransom_note
        .chars()
        .for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] -= 1);
    !hash.iter().any(|&x| x < 0)
}

fn main() {
    assert_eq!(can_construct("a".to_string(), "b".to_string()), false);
    assert_eq!(can_construct("aa".to_string(), "ab".to_string()), false);
    assert_eq!(can_construct("aa".to_string(), "aab".to_string()), true);
}

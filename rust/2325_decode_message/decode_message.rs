/*
 * @Date: 2023-02-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-01
 * @FilePath: /algorithm/rust/2325_decode_message/decode_message.rs
 */

pub fn decode_message(key: String, message: String) -> String {
    use std::collections::HashMap;
    let mut map = HashMap::from([(' ', ' ')]);
    let mut res = String::new();
    let mut cur = 97_u32;
    for c in key.chars() {
        if !map.contains_key(&c) {
            map.insert(c, char::from_u32(cur).unwrap());
            cur += 1;
        }
    }
    for c in message.chars() {
        res.push(*map.get(&c).unwrap());
    }
    res
}

fn main() {
    {
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();
        let ans = "this is a secret".to_string();
        assert_eq!(decode_message(key, message), ans)
    }

    {
        let key = "eljuxhpwnyrdgtqkviszcfmabo".to_string();
        let message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string();
        let ans = "the five boxing wizards jump quickly".to_string();
        assert_eq!(decode_message(key, message), ans)
    }
}

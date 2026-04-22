/*
 * @Date: 2021-12-26 01:06:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-26 01:46:58
 */

pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    text.split(' ')
        .collect::<Vec<&str>>()
        .windows(3)
        .filter(|arr| arr[0] == first && arr[1] == second)
        .map(|arr| arr[2].to_owned())
        .collect()
}

fn main() {
    assert_eq!(
        find_ocurrences(
            "alice is a good girl she is a good student".to_string(),
            "a".to_string(),
            "good".to_string()
        ),
        vec!["girl", "student"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );

    assert_eq!(
        find_ocurrences(
            "we will we will rock you".to_string(),
            "we".to_string(),
            "will".to_string()
        ),
        vec!["we", "rock"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
}

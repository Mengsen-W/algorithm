/*
 * @Date: 2022-05-03 07:51:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-03 08:02:15
 * @FilePath: /algorithm/937_reorder_log_files/reorder_log_files.rs
 */

pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
    logs.sort_by_key(|log| {
        let mut parts = log.splitn(2, ' ');
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        if second.chars().next().unwrap().is_alphabetic() {
            (0, second.to_owned(), first.to_owned())
        } else {
            (1, "".to_owned(), "".to_owned())
        }
    });
    logs
}

fn main() {
    assert_eq!(
        reorder_log_files(vec![
            "dig1 8 1 5 1".to_owned(),
            "let1 art can".to_owned(),
            "dig2 3 6".to_owned(),
            "let2 own kit dig".to_owned(),
            "let3 art zero".to_owned(),
        ]),
        vec![
            "let1 art can".to_owned(),
            "let3 art zero".to_owned(),
            "let2 own kit dig".to_owned(),
            "dig1 8 1 5 1".to_owned(),
            "dig2 3 6".to_owned(),
        ]
    );

    assert_eq!(
        reorder_log_files(vec![
            "a1 9 2 3 1".to_owned(),
            "g1 act car".to_owned(),
            "zo4 4 7".to_owned(),
            "ab1 off key dog".to_owned(),
            "a8 act zoo".to_owned(),
        ]),
        vec![
            "g1 act car".to_owned(),
            "a8 act zoo".to_owned(),
            "ab1 off key dog".to_owned(),
            "a1 9 2 3 1".to_owned(),
            "zo4 4 7".to_owned(),
        ]
    );
}

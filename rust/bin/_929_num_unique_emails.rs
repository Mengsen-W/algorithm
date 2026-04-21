/*
 * @Date: 2022-06-04 09:06:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-04 15:13:19
 * @FilePath: /algorithm/929_num_unique_emails/num_unique_emails.rs
 */

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let mut hash_set = HashSet::new();
    for email in emails {
        let mut v: Vec<&str> = email.splitn(2, '@').collect();
        v[0] = if v[0].contains('+') {
            let v_1: Vec<&str> = v[0].splitn(2, '+').collect();
            v_1[0]
        } else {
            v[0]
        };
        let mut local_name = v[0].to_string();
        local_name.retain(|c| c != '.');
        local_name.push('@');
        local_name.push_str(v[1]);
        hash_set.insert(local_name);
    }
    hash_set.len() as i32
}

fn main() {
    assert_eq!(
        num_unique_emails(
            vec![
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        2
    );

    assert_eq!(
        num_unique_emails(
            vec!["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        3
    );
}

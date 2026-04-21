/*
 * @Date: 2022-01-28 01:23:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-28 01:37:49
 */

pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
    properties.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut st: Vec<i32> = Vec::new();
    let mut ans = 0;
    for p in &properties {
        while !st.is_empty() && st.last().unwrap() < &p[1] {
            ans += 1;
            st.pop();
        }
        st.push(p[1]);
    }
    ans
}

fn main() {
    assert_eq!(
        number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
        0
    );
    assert_eq!(number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]), 1);
    assert_eq!(
        number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
        1
    );
}

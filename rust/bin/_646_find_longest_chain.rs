/*
 * @Date: 2022-09-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-03
 * @FilePath: /algorithm/646_find_longest_chain/find_longest_chain.rs
 */

pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort_by(|a, b| a[1].cmp(&b[1]));
    let (mut ans, mut cur) = (0, i32::MIN);
    pairs.iter().for_each(|pair| {
        if cur < pair[0] {
            cur = pair[1];
            ans += 1;
        }
    });
    ans
}

fn main() {
    assert_eq!(
        find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
}

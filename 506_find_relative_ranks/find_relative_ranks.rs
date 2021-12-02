/*
 * @Date: 2021-12-02 06:06:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-02 06:20:35
 */

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    fn get_name(x: usize) -> String {
        match x {
            1 => "Gold Medal".to_string(),
            2 => "Silver Medal".to_string(),
            3 => "Bronze Medal".to_string(),
            _ => x.to_string(),
        }
    }
    let mut a: Vec<(usize, i32)> = score.into_iter().enumerate().collect();
    let mut ans: Vec<String> = (0..a.len()).map(|_| String::new()).collect();
    a.sort_unstable_by_key(|x| std::cmp::Reverse(x.1));
    a.into_iter().enumerate().for_each(|(i, (idx, _))| {
        ans[idx] = get_name(i + 1);
    });
    ans
}

fn main() {
    assert_eq!(
        find_relative_ranks(vec![5, 4, 3, 2, 1]),
        vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
    );
    assert_eq!(
        find_relative_ranks(vec![10, 3, 8, 9, 4]),
        vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
    );
}

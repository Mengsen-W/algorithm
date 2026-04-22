/*
 * @Date: 2021-12-20 00:46:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-20 01:05:54
 */

pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
    houses.sort_unstable();
    heaters.sort_unstable();
    let (mut ans, mut i, mut j, m, n) = (0, 0, 0, houses.len(), heaters.len());
    while i < m {
        let pre = ans;
        ans = ans.max((houses[i] - heaters[j]).abs());
        while j < n - 1 && (houses[i] - heaters[j]).abs() >= (houses[i] - heaters[j + 1]).abs() {
            j += 1;
            ans = pre.max((houses[i] - heaters[j]).abs());
        }
        i += 1;
    }
    return ans;
}

fn main() {
    assert_eq!(find_radius(vec![1, 2, 3], vec![2]), 1);
    assert_eq!(find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
    assert_eq!(find_radius(vec![1, 5], vec![2]), 3);
}

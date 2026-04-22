/*
 * @Date: 2021-12-03 08:38:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-03 09:40:47
 */

pub fn largest_sum_after_k_negations(nums: Vec<i32>, mut k: i32) -> i32 {
    let mut freq: std::collections::HashMap<i32, i32> =
        nums.iter()
            .fold(std::collections::HashMap::new(), |mut acc, &x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });
    let mut ans = nums.iter().sum::<i32>();
    for i in -100..0 {
        freq.entry(i).or_insert(0);
        if freq.get(&i).unwrap_or(&0) != &0 {
            let ops: i32 = *freq.get(&i).unwrap_or(&0).min(&k);
            ans += (-i) * ops * 2;
            *freq.get_mut(&i).unwrap() -= ops;
            freq.entry(-i).or_insert(0);
            *freq.get_mut(&(-i)).unwrap() += ops;
            k -= ops;
            if k == 0 {
                break;
            }
        }
    }

    if k > 0 && k % 2 == 1 && freq.get(&0).unwrap_or(&0) == &0 {
        for i in 1..=100 {
            if freq.get(&i).unwrap_or(&0) != &0 {
                ans -= i * 2;
                break;
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    assert_eq!(largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
    assert_eq!(largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2), 13);
    assert_eq!(
        largest_sum_after_k_negations(vec![-8, 3, -5, -3, -5, -2], 6),
        22
    );
}

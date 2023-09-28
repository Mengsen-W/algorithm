/*
 * @Date: 2023-09-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-28
 * @FilePath: /algorithm/rust/2251_full_bloom_flowers/full_bloom_flowers.rs
 */

struct Solution;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        use std::collections::BTreeMap;
        let mut diff = BTreeMap::new();
        for f in &flowers {
            *diff.entry(f[0]).or_insert(0) += 1;
            *diff.entry(f[1] + 1).or_insert(0) -= 1;
        }

        let n = people.len();
        let mut id: Vec<usize> = (0..n).collect();
        id.sort_by(|&i, &j| people[i].cmp(&people[j]));

        let mut ans = vec![0; n];
        let mut it = diff.iter().peekable();
        let mut sum = 0;
        for &i in &id {
            while let Some((&t, &d)) = it.peek() {
                if t > people[i] {
                    break;
                }
                sum += d; // 累加不超过 people[i] 的差分值
                it.next();
            }
            ans[i] = sum; // 从而得到这个时刻花的数量
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
            vec![2, 3, 7, 11],
            vec![1, 2, 2, 2],
        ),
        (vec![vec![1, 10], vec![3, 3]], vec![3, 3, 2], vec![2, 2, 1]),
    ];

    for (flowers, people, ans) in tests {
        assert_eq!(Solution::full_bloom_flowers(flowers, people), ans)
    }
}

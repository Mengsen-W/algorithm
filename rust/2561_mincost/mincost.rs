struct Solution;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        use std::collections::HashMap;
        let mut freq = HashMap::new();
        let mut m = i32::MAX;
        for &b in &basket1 {
            *freq.entry(b).or_insert(0) += 1;
            m = m.min(b);
        }
        for &b in &basket2 {
            *freq.entry(b).or_insert(0) -= 1;
            m = m.min(b);
        }

        let mut merge = vec![];
        for (&k, &v) in freq.iter() {
            if v % 2 != 0 {
                return -1;
            }
            for _ in 0..((v as i32).abs() / 2) {
                merge.push(k);
            }
        }

        merge.sort_unstable();
        let res: i64 = merge
            .iter()
            .take(merge.len() / 2)
            .map(|&x| i64::from(x.min(2 * m)))
            .sum();
        res
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 2, 2], vec![1, 4, 1, 2], 1),
        (vec![2, 3, 4, 1], vec![3, 2, 5, 1], -1),
    ];

    for (basket1, basket2, ans) in tests {
        assert_eq!(Solution::min_cost(basket1, basket2), ans);
    }
}

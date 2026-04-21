/*
 * @Date: 2023-01-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-22
 * @FilePath: /algorithm/rust/1815_max_happy_groups/max_happy_groups.rs
 */

pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let k_width: i32 = 5;
    let k_width_mask: i32 = (1 << k_width) - 1;
    let mut cnt = vec![0; batch_size as usize];
    for x in groups {
        cnt[(x % batch_size) as usize] += 1;
    }

    let mut start: i64 = 0;
    for i in (1..=(batch_size - 1)).rev() {
        start = (start << k_width) | cnt[i as usize];
    }

    let mut memo: HashMap<i64, i32> = HashMap::new();

    fn dfs(
        mask: i64,
        memo: &mut HashMap<i64, i32>,
        batch_size: i32,
        k_width: i32,
        k_width_mask: i32,
    ) -> i32 {
        if mask == 0 {
            return 0;
        }
        if memo.contains_key(&mask) {
            return *memo.get(&mask).unwrap();
        }
        let mut total = 0;
        for i in 1..batch_size {
            let amount = (mask >> ((i - 1) * k_width)) & k_width_mask as i64;
            total += i * amount as i32;
        }
        let mut best = 0;
        for i in 1..batch_size {
            let amount = (mask >> ((i - 1) * k_width)) & k_width_mask as i64;
            if amount > 0 {
                let mut result = dfs(
                    mask - (1 << ((i - 1) * k_width)),
                    memo,
                    batch_size,
                    k_width,
                    k_width_mask,
                );
                if (total - i) % batch_size == 0 {
                    result += 1;
                }
                best = best.max(result);
            }
        }
        memo.entry(mask)
            .and_modify(|counter| *counter = best)
            .or_insert(best);
        best
    }
    dfs(start, &mut memo, batch_size, k_width, k_width_mask) + cnt[0] as i32
}

fn main() {
    {
        let batch_size = 3;
        let groups = vec![1, 2, 3, 4, 5, 6];
        let ans = 4;
        assert_eq!(max_happy_groups(batch_size, groups), ans);
    }

    {
        let batch_size = 4;
        let groups = vec![1, 3, 2, 5, 2, 2, 1, 6];
        let ans = 4;
        assert_eq!(max_happy_groups(batch_size, groups), ans);
    }
}

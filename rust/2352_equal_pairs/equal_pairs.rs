/*
 * @Date: 2023-06-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-06
 * @FilePath: /algorithm/rust/2352_equal_pairs/equal_pairs.rs
 */

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let (mut cnt, mut ans, n) = (std::collections::HashMap::new(), 0, grid.len());
    grid.iter().for_each(|row| {
        *cnt.entry(row.to_owned()).or_insert(0) += 1;
    });
    (0..n).for_each(|i| {
        let mut col = vec![0; n];
        grid.iter().enumerate().for_each(|(u, row)| {
            col[u] = *row.get(i).unwrap();
        });
        if let Some(v) = cnt.get(&col) {
            ans += *v;
        }
    });

    ans
}

fn main() {
    {
        let grid = [[3, 2, 1], [1, 7, 6], [2, 7, 7]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = 1;
        assert_eq!(equal_pairs(grid), ans);
    }

    {
        let grid = [[3, 1, 2, 2], [1, 4, 4, 5], [2, 4, 2, 2], [2, 4, 2, 2]]
            .iter()
            .map(|v| v.to_vec())
            .collect();
        let ans = 3;
        assert_eq!(equal_pairs(grid), ans);
    }
}

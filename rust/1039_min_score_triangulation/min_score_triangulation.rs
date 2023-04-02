/*
 * @Date: 2023-04-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-02
 * @FilePath: /algorithm/rust/1039_min_score_triangulation/min_score_triangulation.rs
 */

pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
    fn dfs(i: usize, j: usize, v: &[i32], tab: &mut Vec<Vec<i32>>) -> i32 {
        if tab[i][j] != i32::MAX {
            return tab[i][j];
        }
        for k in i + 1..j {
            tab[i][j] = tab[i][j].min(dfs(i, k, v, tab) + dfs(k, j, v, tab) + v[i] * v[k] * v[j]);
        }
        tab[i][j]
    }
    let n = values.len();
    let mut tab = vec![vec![i32::MAX; n]; n];
    for i in 0..n - 1 {
        tab[i][i + 1] = 0; // 两个点无法组成三角形
    }
    dfs(0, n - 1, &values, &mut tab)
}

fn main() {
    {
        let values = vec![1, 2, 3];
        let ans = 6;
        assert_eq!(min_score_triangulation(values), ans);
    }

    {
        let values = vec![3, 7, 4, 5];
        let ans = 144;
        assert_eq!(min_score_triangulation(values), ans);
    }

    {
        let values = vec![1, 3, 1, 4, 1, 5];
        let ans = 13;
        assert_eq!(min_score_triangulation(values), ans);
    }
}

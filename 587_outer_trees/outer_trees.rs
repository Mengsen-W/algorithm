/*
 * @Date: 2022-04-23 09:49:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-23 11:11:06
 * @FilePath: /algorithm/587_outer_trees/outer_trees.rs
 */

pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn cross(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
        (q[0] - p[0]) * (r[1] - q[1]) - (q[1] - p[1]) * (r[0] - q[0])
    }

    let n = trees.len();
    if n < 4 {
        return trees;
    }
    trees.sort_unstable_by(|a, b| {
        if a[0] == b[0] {
            a[1].cmp(&b[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    let mut hull = Vec::new();
    let mut used = vec![false; n];
    hull.push(0);
    for i in 1..n {
        while hull.len() > 1
            && cross(
                &trees[hull[hull.len() - 2]],
                &trees[*(hull.last().unwrap()) as usize],
                &trees[i],
            ) < 0
        {
            used[*(hull.last().unwrap()) as usize] = false;
            hull.pop();
        }
        used[i] = true;
        hull.push(i);
    }

    let m = hull.len();
    for i in (0..=n - 2).rev() {
        if !used[i] {
            while hull.len() > m
                && cross(
                    &trees[hull[hull.len() - 2]],
                    &trees[*(hull.last().unwrap()) as usize],
                    &trees[i],
                ) < 0
            {
                used[*(hull.last().unwrap()) as usize] = false;
                hull.pop();
            }
            used[i] = true;
            hull.push(i);
        }
    }

    hull.pop();
    let mut res = Vec::new();
    for v in hull {
        res.push(trees[v].clone());
    }
    res
}

fn main() {
    assert_eq!(
        outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2]
        ]),
        vec![vec![1, 1], vec![2, 0], vec![4, 2], vec![3, 3], vec![2, 4]]
    );
    assert_eq!(
        outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2],]),
        vec![vec![1, 2], vec![2, 2], vec![4, 2]]
    );
}

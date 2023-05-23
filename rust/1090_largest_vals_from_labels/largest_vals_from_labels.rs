/*
 * @Date: 2023-05-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-23
 * @FilePath: /algorithm/rust/1090_largest_vals_from_labels/largest_vals_from_labels.rs
 */

pub fn largest_vals_from_labels(
    values: Vec<i32>,
    labels: Vec<i32>,
    num_wanted: i32,
    use_limit: i32,
) -> i32 {
    use std::collections::HashMap;
    let n = values.len();
    let mut v: Vec<Vec<i32>> = vec![];
    for i in 0..n {
        v.push(vec![values[i], labels[i]]);
    }
    v.sort_by(|a, b| b[0].cmp(&a[0]));
    let mut res = 0;
    let mut m: HashMap<i32, i32> = HashMap::new();
    let mut cnt = 0;
    for i in 0..n {
        if (m.contains_key(&v[i][1]) && m[&v[i][1]] < use_limit) || !m.contains_key(&v[i][1]) {
            res += v[i][0];
            cnt += 1;
            if m.contains_key(&v[i][1]) {
                m.insert(v[i][1], m[&v[i][1]] + 1);
            } else {
                m.insert(v[i][1], 1);
            }
        }
        if cnt == num_wanted {
            break;
        }
    }
    res
}

fn main() {
    {
        let values = vec![5, 4, 3, 2, 1];
        let labels = vec![1, 1, 2, 2, 3];
        let num_wanted = 3;
        let use_limit = 1;
        let ans = 9;
        assert_eq!(
            largest_vals_from_labels(values, labels, num_wanted, use_limit),
            ans
        );
    }

    {
        let values = vec![5, 4, 3, 2, 1];
        let labels = vec![1, 3, 3, 3, 2];
        let num_wanted = 3;
        let use_limit = 2;
        let ans = 12;
        assert_eq!(
            largest_vals_from_labels(values, labels, num_wanted, use_limit),
            ans
        );
    }

    {
        let values = vec![9, 8, 8, 7, 6];
        let labels = vec![0, 0, 0, 1, 1];
        let num_wanted = 3;
        let use_limit = 1;
        let ans = 16;
        assert_eq!(
            largest_vals_from_labels(values, labels, num_wanted, use_limit),
            ans
        );
    }
}

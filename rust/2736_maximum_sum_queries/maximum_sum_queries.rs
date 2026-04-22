/*
 * @Date: 2023-11-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-18
 * @FilePath: /algorithm/rust/2736_maximum_sum_queries/maximum_sum_queries.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_sum_queries(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut a: Vec<(i32, i32)> = nums1.into_iter().zip(nums2.into_iter()).collect();
        a.sort_by(|x, y| y.0.cmp(&x.0));

        let mut qid: Vec<usize> = (0..queries.len()).collect();
        qid.sort_by(|&i, &j| queries[j][0].cmp(&queries[i][0]));

        let mut ans = vec![-1; queries.len()];
        let mut st: Vec<(i32, i32)> = Vec::new();
        let mut j = 0;
        for &i in &qid {
            let x = queries[i][0];
            let y = queries[i][1];
            while j < a.len() && a[j].0 >= x {
                // 下面只需关心 a[j].1
                while !st.is_empty() && st.last().unwrap().1 <= a[j].0 + a[j].1 {
                    // a[j].1 >= st.last().unwrap().0
                    st.pop();
                }
                if st.is_empty() || st.last().unwrap().0 < a[j].1 {
                    st.push((a[j].1, a[j].0 + a[j].1));
                }
                j += 1;
            }
            let p = st.partition_point(|&p| p.0 < y);
            if p < st.len() {
                ans[i] = st[p].1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![4, 3, 1, 2],
            vec![2, 4, 9, 5],
            vec![vec![4, 1], vec![1, 3], vec![2, 5]],
            vec![6, 10, 7],
        ),
        (
            vec![3, 2, 5],
            vec![2, 3, 4],
            vec![vec![4, 4], vec![3, 2], vec![1, 1]],
            vec![9, 9, 9],
        ),
        (vec![2, 1], vec![2, 3], vec![vec![3, 3]], vec![-1]),
    ];

    for (nums1, nums2, queries, ans) in tests {
        assert_eq!(Solution::maximum_sum_queries(nums1, nums2, queries), ans);
    }
}

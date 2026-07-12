struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return vec![];
        }
        let mut a = arr
            .iter()
            .enumerate()
            .map(|(i, &n)| (n, i))
            .collect::<Vec<_>>();
        a.sort_unstable();
        let mut ans = vec![0; a.len()];
        ans[a[0].1] = 1;
        a.windows(2).for_each(|x| {
            ans[x[1].1] = ans[x[0].1] + if x[0].0 == x[1].0 { 0 } else { 1 };
        });
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![40, 10, 20, 30], vec![4, 1, 2, 3]),
        (vec![100, 100, 100], vec![1, 1, 1]),
        (
            vec![37, 12, 28, 9, 100, 56, 80, 5, 12],
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3],
        ),
    ];

    for (arr, ans) in tests {
        assert_eq!(Solution::array_rank_transform(arr), ans);
    }
}

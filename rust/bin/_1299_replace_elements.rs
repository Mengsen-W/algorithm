struct Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; arr.len()];
        let n = arr.len();
        ans[n - 1] = -1;
        for i in (0..n - 1).rev() {
            ans[i] = ans[i + 1].max(arr[i + 1]);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![17, 18, 5, 4, 6, 1], vec![18, 6, 6, 6, 1, -1]),
        (vec![400], vec![-1]),
    ];

    for (arr, ans) in tests {
        assert_eq!(Solution::replace_elements(arr), ans);
    }
}

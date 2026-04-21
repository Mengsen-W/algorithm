struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }
        for i in 0..=n - 3 {
            if (arr[i] & 1) != 0 && (arr[i + 1] & 1) != 0 && (arr[i + 2] & 1) != 0 {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let tests = vec![
        (vec![2, 6, 4, 1], false),
        (vec![1, 2, 34, 3, 4, 5, 7, 23, 12], true),
    ];

    for (arr, ans) in tests {
        assert_eq!(Solution::three_consecutive_odds(arr), ans);
    }
}

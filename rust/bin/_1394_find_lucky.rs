struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        for x in arr {
            *m.entry(x).or_insert(0) += 1;
        }
        let mut ans = -1;
        for (&key, &value) in m.iter() {
            if key == value {
                ans = ans.max(key);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![2, 2, 3, 4], 2),
        (vec![1, 2, 2, 3, 3, 3], 3),
        (vec![2, 2, 2, 3, 3], -1),
        (vec![5], -1),
        (vec![7, 7, 7, 7, 7, 7, 7], 7),
    ];

    for (arr, expected) in tests {
        assert_eq!(Solution::find_lucky(arr), expected);
    }
}

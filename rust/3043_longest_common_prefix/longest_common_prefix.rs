struct Solution;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut seen = HashSet::new();

        for mut num in arr1 {
            while num > 0 {
                seen.insert(num);
                num /= 10;
            }
        }

        let mut best = 0;
        for mut num in arr2 {
            while num > 0 {
                if seen.contains(&num) {
                    best = best.max(num);
                }
                num /= 10;
            }
        }

        if best == 0 {
            0
        } else {
            best.to_string().len() as i32
        }
    }
}

fn main() {
    let tests = vec![
        (vec![1, 10, 100], vec![1000], 3),
        (vec![1, 2, 3], vec![4, 4, 4], 0),
    ];

    for (arr1, arr2, expected) in tests {
        assert_eq!(Solution::longest_common_prefix(arr1, arr2), expected);
    }
}

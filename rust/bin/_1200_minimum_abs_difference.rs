struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let min = arr
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .min()
            .unwrap();
        arr.windows(2)
            .filter_map(|slice| {
                if slice[1] - slice[0] == min {
                    Some(slice.to_vec())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 1, 3], vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        (vec![1, 3, 6, 10, 15], vec![vec![1, 3]]),
        (
            vec![3, 8, -10, 23, 19, -4, -14, 27],
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]],
        ),
    ];

    for (arr, expected) in tests {
        assert_eq!(Solution::minimum_abs_difference(arr), expected);
    }
}

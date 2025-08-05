struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut count = 0;
        let n = baskets.len();
        for fruit in fruits {
            let mut unset = 1;
            for i in 0..n {
                if fruit <= baskets[i] {
                    baskets[i] = 0;
                    unset = 0;
                    break;
                }
            }
            count += unset;
        }
        count
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 5], vec![3, 5, 4], 1),
        (vec![3, 6, 1], vec![6, 4, 7], 0),
    ];

    for (fruits, baskets, expected) in tests {
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), expected);
    }
}

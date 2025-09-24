struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        use std::cmp::max;
        let n = baskets.len();
        let mut baskets = baskets;
        let m = (n as f64).sqrt() as usize;
        let section = (n + m - 1) / m;
        let mut count = 0;
        let mut max_v = vec![0; section];

        for i in 0..n {
            let sec = i / m;
            max_v[sec] = max(max_v[sec], baskets[i])
        }

        for &fruit in &fruits {
            let mut unset = 1;
            for sec in 0..section {
                if max_v[sec] < fruit {
                    continue;
                }

                let mut choose = false;
                max_v[sec] = 0;
                for i in 0..m {
                    let pos = sec * m + i;
                    if pos < n && baskets[pos] >= fruit && !choose {
                        baskets[pos] = 0;
                        choose = true;
                    }
                    if pos < n {
                        max_v[sec] = max(max_v[sec], baskets[pos]);
                    }
                }
                unset = 0;
                break;
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

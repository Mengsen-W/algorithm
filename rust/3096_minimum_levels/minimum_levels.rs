struct Solution;

impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let n = possible.len() as i32;
        let tot: i32 = possible.iter().sum::<i32>() * 2 - n;
        let mut pre = 0;
        for (i, &val) in possible.iter().enumerate().take(n as usize - 1) {
            pre += if val == 1 { 1 } else { -1 };
            if 2 * pre > tot {
                return (i + 1) as i32;
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 0, 1, 0], 1),
        (vec![1, 1, 1, 1, 1], 3),
        (vec![0, 0], -1),
    ];

    for (possible, ans) in tests {
        assert_eq!(Solution::minimum_levels(possible), ans);
    }
}

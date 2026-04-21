struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut count: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut res = 0;
        for i in low_limit..=high_limit {
            let mut b = 0;
            let mut x = i;
            while x != 0 {
                b += x % 10;
                x /= 10;
            }
            count
                .entry(b)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            res = res.max(*count.get(&b).unwrap_or(&0));
        }
        res
    }
}

fn main() {
    let tests = vec![(1, 10, 2), (5, 15, 2), (19, 28, 2)];

    for (low_limit, high_limit, ans) in tests {
        assert_eq!(Solution::count_balls(low_limit, high_limit), ans);
    }
}

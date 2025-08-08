struct Solution;

impl Solution {
    pub fn dfs(
        memo: &mut Vec<Vec<Option<f64>>>,
        a: i32,
        b: i32,
        opt: &impl Fn(i32, i32) -> i32,
    ) -> f64 {
        // base case
        if a == 0 && b != 0 {
            return 1.0;
        }
        if a == 0 && b == 0 {
            return 0.5;
        }
        if a != 0 && b == 0 {
            return 0.0;
        }
        match memo[a as usize][b as usize] {
            Some(prob) => {
                return prob;
            }
            None => {}
        }
        let mut prob = 0.0;
        for i in 0..=3 {
            let amount = (i as i32) * 25;
            prob += 0.25 * Solution::dfs(memo, opt(a, 100 - amount), opt(b, amount), opt);
        }
        memo[a as usize][b as usize] = Some(prob);
        return prob;
    }

    pub fn soup_servings(n: i32) -> f64 {
        if n > 5000 {
            return 1.0;
        }
        let mut memo = vec![vec![None; n as usize + 1]; n as usize + 1];
        let subtract = |x: i32, y: i32| {
            return x - x.min(y);
        };
        Solution::dfs(&mut memo, n, n, &subtract)
    }
}

fn main() {
    let tests = vec![(50, 0.62500), (100, 0.71875)];

    for (n, expected) in tests {
        assert_eq!(
            Solution::soup_servings(n),
            expected,
            "soup_servings({}) should be {}",
            n,
            expected
        );
    }
}

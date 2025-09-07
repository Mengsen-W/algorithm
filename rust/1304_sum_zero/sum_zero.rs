struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(n as usize);
        for i in 1..=n / 2 {
            ans.push(i);
            ans.push(-i);
        }
        if n % 2 == 1 {
            ans.push(0);
        }
        ans
    }
}

fn check(ans: &Vec<i32>) -> bool {
    use std::collections::HashSet;
    let unique_set: HashSet<i32> = ans.iter().cloned().collect();
    if unique_set.len() != ans.len() {
        return false;
    }
    if unique_set.iter().sum::<i32>() != 0 {
        return false;
    }
    true
}

fn main() {
    let tests = vec![5, 3, 1];
    for n in tests {
        let ans = Solution::sum_zero(n);
        assert_eq!(check(&ans), true);
    }
}

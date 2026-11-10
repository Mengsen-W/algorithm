struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let (mut a, mut b, mut c) = (0, 0, 0);
        for i in 0..n as usize {
            let nxt = (a + 9).min(b + 4).min(c + 1) + costs[i];
            a = b;
            b = c;
            c = nxt;
        }
        c
    }
}

fn main() {
    let tests = vec![
        (4, vec![1, 2, 3, 4], 13),
        (4, vec![5, 1, 6, 2], 11),
        (3, vec![9, 8, 3], 12),
    ];

    for (n, costs, ans) in tests {
        assert_eq!(Solution::climb_stairs(n, costs), ans);
    }
}

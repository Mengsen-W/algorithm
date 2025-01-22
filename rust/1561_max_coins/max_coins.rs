struct Solution;

impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        piles.sort();
        let length = piles.len();
        let rounds = length / 3;
        let mut coins = 0;
        let mut index = length - 2;
        for _ in 0..rounds {
            coins += piles[index];
            index -= 2;
        }
        coins
    }
}

fn main() {
    let tests = vec![
        (vec![2, 4, 1, 2, 7, 8], 9),
        (vec![2, 4, 5], 4),
        (vec![9, 8, 7, 6, 5, 1, 2, 3, 4], 18),
    ];

    for (piles, ans) in tests {
        assert_eq!(Solution::max_coins(piles), ans);
    }
}

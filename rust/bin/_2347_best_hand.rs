/*
 * @Date: 2023-02-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-20
 * @FilePath: /algorithm/rust/2347_best_hand/best_hand.rs
 */

pub fn best_hand(mut ranks: Vec<i32>, suits: Vec<char>) -> String {
    if suits.iter().all(|&x| x == suits[0]) {
        "Flush"
    } else {
        ranks.sort_unstable();
        let c = ranks.windows(2).fold((0, 0), |mut s, x| {
            if x[0] == x[1] {
                s.1 += 1
            } else {
                s.0 = s.0.max(s.1);
                s.1 = 0
            };
            s
        });
        match c.0.max(c.1) {
            0 => "High Card",
            1 => "Pair",
            _ => "Three of a Kind",
        }
    }
    .to_string()
}

fn main() {
    {
        let ranks = vec![13, 2, 3, 1, 9];
        let suits = vec!['a', 'a', 'a', 'a', 'a'];
        let ans = "Flush".to_string();
        assert_eq!(best_hand(ranks, suits), ans);
    }

    {
        let ranks = vec![10, 10, 2, 12, 9];
        let suits = vec!['a', 'b', 'c', 'a', 'd'];
        let ans = "Pair".to_string();
        assert_eq!(best_hand(ranks, suits), ans);
    }

    {
        let ranks = vec![4, 4, 2, 4, 4];
        let suits = vec!['d', 'a', 'a', 'b', 'c'];
        let ans = "Three of a Kind".to_string();
        assert_eq!(best_hand(ranks, suits), ans);
    }
}

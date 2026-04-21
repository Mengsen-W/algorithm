struct Solution;

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        const EPS: f64 = 1e-9;

        fn dfs(cards: Vec<f64>) -> bool {
            let n = cards.len();
            if n == 1 {
                return (cards[0] - 24.0).abs() < EPS;
            }

            // 选两张牌 x=cards[i] 和 y=cards[j]
            for (i, x) in cards.iter().enumerate() {
                for j in i + 1..n {
                    let y = cards[j];

                    // 六种情况：加减乘除，其中减和除都有两种不同的顺序
                    let mut candidates = vec![x + y, x - y, y - x, x * y];
                    if y.abs() > EPS {
                        candidates.push(x / y);
                    }
                    if x.abs() > EPS {
                        candidates.push(y / x);
                    }

                    for res in candidates {
                        let mut new_cards = cards.clone();
                        new_cards.remove(j); // 删除 j
                        new_cards[i] = res; // 覆盖 i
                        if dfs(new_cards) {
                            return true;
                        }
                    }
                }
            }
            false
        }

        let a = cards.into_iter().map(|x| x as f64).collect::<Vec<_>>();
        dfs(a)
    }
}

fn main() {
    let tests = vec![(vec![4, 1, 8, 7], true), (vec![1, 2, 1, 2], false)];

    for (cards, expected) in tests {
        assert_eq!(Solution::judge_point24(cards), expected);
    }
}

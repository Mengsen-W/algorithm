struct Solution;

impl Solution {
    fn solve(num: i64) -> i64 {
        // 数字小于 3 位波动值为 0
        if num < 100 {
            return 0;
        }
        let s = num.to_string();
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();

        #[derive(Clone)]
        struct State {
            prev: usize,
            curr: usize,
            tight: usize,
            lead: usize,
            cnt: i64,
            sum: i64,
        }

        // 数位 10 表示存在前导零时的无效状态
        let mut curr_states = vec![State {
            prev: 10,
            curr: 10,
            tight: 1,
            lead: 1,
            cnt: 1,
            sum: 0,
        }];

        for pos in 0..n {
            let limit = (chars[pos] as u8 - b'0') as usize;
            let mut cnt = [[[[0i64; 11]; 11]; 2]; 2];
            let mut sum_arr = [[[[0i64; 11]; 11]; 2]; 2];

            for st in &curr_states {
                let max_digit = if st.tight == 1 { limit } else { 9 };
                for digit in 0..=max_digit {
                    let new_lead = if st.lead == 1 && digit == 0 { 1 } else { 0 };
                    let new_prev = st.curr;
                    let new_curr = if new_lead == 1 { 10 } else { digit };
                    let new_tight = if st.tight == 1 && digit == max_digit {
                        1
                    } else {
                        0
                    };

                    let mut add = 0;
                    // 已有三位有效数字时才计算波动（prev和curr都有效，且不是前导零）
                    if new_lead == 0 && st.prev != 10 && st.curr != 10 {
                        if (st.prev < st.curr && st.curr > digit)
                            || (st.prev > st.curr && st.curr < digit)
                        {
                            add = st.cnt;
                        }
                    }

                    cnt[new_tight][new_lead][new_prev][new_curr] += st.cnt;
                    sum_arr[new_tight][new_lead][new_prev][new_curr] += st.sum + add;
                }
            }

            // 收集合法状态
            let mut next_states = Vec::new();
            for tight in 0..2 {
                for lead in 0..2 {
                    for prev in 0..=10 {
                        for curr in 0..=10 {
                            let c = cnt[tight][lead][prev][curr];
                            if c != 0 {
                                next_states.push(State {
                                    prev,
                                    curr,
                                    tight,
                                    lead,
                                    cnt: c,
                                    sum: sum_arr[tight][lead][prev][curr],
                                });
                            }
                        }
                    }
                }
            }
            curr_states = next_states;
        }

        // 累加所有合法状态的波动值之和
        let mut ans = 0;
        for st in curr_states {
            ans += st.sum;
        }
        ans
    }

    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        Self::solve(num2) - Self::solve(num1 - 1)
    }
}

fn main() {
    let tests = vec![(120, 130, 3), (198, 202, 3), (4848, 4848, 2)];

    for (num1, num2, expected) in tests {
        assert_eq!(Solution::total_waviness(num1, num2), expected);
    }
}

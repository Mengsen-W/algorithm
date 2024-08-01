struct Solution;

impl Solution {
    pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
        let mut cnt = cnt;
        let mut val = vec![0; 1005];
        for &card in &cards {
            val[card as usize] += 1;
        }
        let mut ans = 0;
        let mut tmp = 0;
        let mut ed = -1;
        let mut odd = -1;
        let mut even = -1;
        for i in (1..=1000).rev() {
            if val[i] == 0 {
                continue;
            }
            if val[i] > cnt {
                tmp += cnt * i as i32;
                cnt = 0;
            } else {
                tmp += val[i] * i as i32;
                cnt -= val[i];
                val[i] = 0;
            }
            if i % 2 == 1 {
                odd = i as i32;
            } else {
                even = i as i32;
            }
            if cnt == 0 {
                if val[i] > 0 {
                    ed = i as i32;
                } else {
                    ed = (i - 1) as i32;
                }
                break;
            }
        }
        if tmp % 2 == 0 {
            return tmp;
        }
        for i in (1..=ed as usize).rev() {
            if val[i] == 0 {
                continue;
            }
            if i % 2 == 1 {
                if even != -1 {
                    ans = ans.max(tmp - even + i as i32);
                }
            } else {
                if odd != -1 {
                    ans = ans.max(tmp - odd + i as i32);
                }
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 8, 9], 3, 18), (vec![3, 3, 1], 1, 0)];

    for (cards, cnt, ans) in tests {
        assert_eq!(Solution::maxmium_score(cards, cnt), ans);
    }
}

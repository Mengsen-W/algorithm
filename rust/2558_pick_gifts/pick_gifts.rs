/*
 * @Date: 2023-10-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-28
 * @FilePath: /algorithm/rust/2558_pick_gifts/pick_gifts.rs
 */

struct Solution;

fn isqrt(mut n: i32) -> i32 {
    let mut m = 1 << (i32::BITS - 2);
    let mut s = 0;
    while m != 0 {
        let b = s | m;
        s >>= 1;
        if n >= b {
            n -= b;
            s |= m;
        }
        m >>= 2;
    }
    s
}

impl Solution {
    pub fn pick_gifts(mut gifts: Vec<i32>, k: i32) -> i64 {
        gifts.sort_unstable();
        gifts.reverse();
        gifts.push(0);
        let sum = gifts.iter().fold(0i64, |acc, &x| acc + (x as i64));
        let (mut ret, mut cnt, mut m, mut level) = (sum, k, 0, gifts[0]);
        'a: while level > 1 {
            level = isqrt(level);
            let (mut i, mut j, mut merge) = (0, m, vec![]);
            while cnt > 0 {
                let loc = match (i < m, gifts[j] >= level) {
                    (true, true) => {
                        if gifts[i] < gifts[j] {
                            &mut j
                        } else {
                            &mut i
                        }
                    }
                    (true, false) => &mut i,
                    (false, true) => &mut j,
                    _ => {
                        m = j;
                        gifts[..m].copy_from_slice(&merge);
                        continue 'a;
                    }
                };
                let v = gifts[*loc];
                *loc += 1;
                cnt -= 1;
                let sqv = isqrt(v);
                ret -= (v - sqv) as i64;
                merge.push(sqv);
            }
            break;
        }
        ret
    }
}

fn main() {
    let tests = vec![(vec![25, 64, 9, 4, 100], 4, 29), (vec![1, 1, 1, 1], 4, 4)];

    for (gifts, k, ans) in tests {
        assert_eq!(Solution::pick_gifts(gifts, k), ans);
    }
}

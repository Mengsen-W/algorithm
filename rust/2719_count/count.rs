/*
 * @Date: 2024-01-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-16
 * @FilePath: /algorithm/rust/2719_count/count.rs
 */

struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn solve(
        pos: usize,
        lower: bool,
        higher: bool,
        dig_sum: i16,
        num1: &Vec<i16>,
        num2: &Vec<i16>,
        min_sum: i16,
        max_sum: i16,
        cache: &mut BTreeMap<(usize, bool, bool, i16), i64>,
    ) -> i64 {
        const MOD: i64 = 1000000007;
        // println!("{} ", pos);
        if pos == num1.len() {
            if dig_sum >= min_sum && dig_sum <= max_sum {
                return 1;
            } else {
                return 0;
            }
        }
        // if (pos==num1.len() && dig_sum>= num1.iter().sum() && dig_sum<=num2.iter().sum()){
        //     return 1;
        // }

        if cache.contains_key(&(pos, lower, higher, dig_sum)) {
            return *cache.get(&(pos, lower, higher, dig_sum)).unwrap();
        }

        let lo = if lower == false { num1[pos] } else { 0 };
        let hi = if higher == false { num2[pos] } else { 9 };
        // println!("pos {}, lo {}, hi {}", pos, lo, hi);
        let mut count: i64 = 0;

        for idx in lo..=hi {
            if dig_sum + idx <= max_sum {
                // println!("pos {},  idx {}", pos, idx);
                count = (count
                    + Self::solve(
                        pos + 1,
                        lower || (idx > lo),
                        higher || (idx < hi),
                        dig_sum + idx,
                        num1,
                        num2,
                        min_sum,
                        max_sum,
                        cache,
                    ))
                    % MOD;
            }
        }

        cache.entry((pos, lower, higher, dig_sum)).or_insert(count);
        count
    }

    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let mut _num1 = String::from("");
        // let mut num2 = String::from("");
        let mut leading_zeros = num2.len() - num1.len();
        while leading_zeros > 0 {
            _num1.push_str("0");
            leading_zeros -= 1;
        }
        _num1.push_str(num1.as_str());
        let num1 = _num1;

        // println!("num1 {:?} ", num1);
        // println!("num2 {:?} ", num2);
        let mut cache: BTreeMap<(usize, bool, bool, i16), i64> = BTreeMap::new();

        let num1 = num1
            .chars()
            .map(|c| (c.to_digit(10).unwrap() as i16))
            .collect::<Vec<i16>>();
        let num2 = num2
            .chars()
            .map(|c| (c.to_digit(10).unwrap() as i16))
            .collect::<Vec<i16>>();
        // let num2 = num2.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<i16>>();

        Self::solve(
            0,
            false,
            false,
            0,
            &num1,
            &num2,
            min_sum as i16,
            max_sum as i16,
            &mut cache,
        ) as i32
    }
}

fn main() {
    let tests = vec![("1", "12", 1, 8, 11), ("1", "5", 1, 5, 5)];

    for (num1, num2, min_sum, max_sum, ans) in tests {
        assert_eq!(
            Solution::count(num1.to_string(), num2.to_string(), min_sum, max_sum),
            ans
        );
    }
}

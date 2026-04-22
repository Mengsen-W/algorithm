struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        // C 是数组 nums 元素的上限，C3 是 C 的立方根
        const C: i32 = 100000;
        const C3: i32 = 46;

        let mut isprime = vec![0; (C + 1) as usize];
        let mut primes = Vec::new();

        // 埃拉托斯特尼筛法
        for i in 2..=C {
            isprime[i as usize] = 1;
        }
        for i in 2..=C {
            if isprime[i as usize] == 1 {
                primes.push(i);
            }
            let mut j = i + i;
            while j <= C {
                isprime[j as usize] = 0;
                j += i;
            }
        }

        // 欧拉筛法
        /*
        for i in 2..=C {
            if isprime[i as usize] == 1 {
                primes.push(i);
            }
            for &prime in &primes {
                if i * prime > C {
                    break;
                }
                isprime[(i * prime) as usize] = 0;
                if i % prime == 0 {
                    break;
                }
            }
        }
        */

        // 通过质数表构造出所有的四因数
        let mut factor4 = HashMap::new();
        for &prime in &primes {
            if prime <= C3 {
                let key = prime * prime * prime;
                let value = 1 + prime + prime * prime + prime * prime * prime;
                factor4.insert(key, value);
            }
        }
        for i in 0..primes.len() {
            for j in i + 1..primes.len() {
                if primes[i] <= C / primes[j] {
                    let key = primes[i] * primes[j];
                    let value = 1 + primes[i] + primes[j] + primes[i] * primes[j];
                    factor4.insert(key, value);
                } else {
                    break;
                }
            }
        }

        let mut ans = 0;
        for num in nums {
            if let Some(&value) = factor4.get(&num) {
                ans += value;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![21, 4, 7], 32),
        (vec![21, 21], 64),
        (vec![1, 2, 3, 4, 5], 0),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::sum_four_divisors(nums), expected);
    }
}

/*
 * @Date: 2022-02-22 01:08:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-22 02:54:40
 */

pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
    const PRIMES: [usize; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    const NUM_MAX: usize = 30;
    const MOD_: i64 = 1000000007;

    let freq: Vec<i64> = nums
        .iter()
        .map(|&x| x)
        .fold(vec![0; NUM_MAX + 1], |mut acc, x| {
            acc[x as usize] += 1;
            acc
        });

    let mut f: Vec<i64> = vec![0; 1 << PRIMES.len()];
    f[0] = 1;
    (0..freq[1]).for_each(|_| f[0] = f[0] * 2 % MOD_);

    for i in 2..=NUM_MAX {
        if freq[i] == 0 {
            continue;
        }

        let (mut subset, x, mut check) = (0, i, true);
        for j in 0..PRIMES.len() {
            let prime = PRIMES[j];
            if x % (prime * prime) == 0 {
                check = false;
                break;
            }

            if x % prime == 0 {
                subset |= 1 << j;
            }
        }
        if !check {
            continue;
        }

        for mask in (1..=(1 << PRIMES.len())).rev() {
            if mask & subset == subset {
                f[mask] = f[mask] + f[mask ^ subset] * freq[x] % MOD_;
            }
        }
    }

    let mask_max = 1 << PRIMES.len();
    (1..mask_max).fold(0, |mut ans, mask| {
        ans = (ans + f[mask]) % MOD_;
        ans
    }) as i32
}

fn main() {
    assert_eq!(number_of_good_subsets(vec![1, 2, 3, 4]), 6);
    assert_eq!(number_of_good_subsets(vec![4, 2, 3, 15]), 5);
    assert_eq!(
        number_of_good_subsets(vec![
            10, 11, 5, 1, 10, 1, 3, 1, 26, 11, 6, 1, 1, 15, 1, 7, 22, 1, 1, 1, 1, 1, 23, 1, 29, 5,
            6, 1, 1, 29, 1, 1, 21, 19, 1, 1, 1, 2, 1, 11, 1, 15, 1, 22, 14, 1, 1, 1, 1, 6, 7, 1,
            14, 3, 5, 1, 22, 1, 1, 1, 17, 1, 29, 2, 1, 15, 10, 1, 5, 7, 1, 1, 1, 30, 1, 30, 1, 21,
            10, 1, 1, 1, 1, 1, 2, 6, 5, 7, 3, 1, 1, 19, 29, 1, 7, 13, 14, 1, 5, 26, 19, 11, 1, 1,
            1, 1, 1, 1, 1, 1, 22, 15, 1, 1, 13, 1, 17, 1, 1, 1, 13, 6, 1, 10, 1, 1, 17, 1, 1, 3,
            14, 7, 17, 1, 13, 1, 1, 1, 1, 1, 11, 1, 1, 6, 1, 1, 1, 1, 1, 2, 1, 30, 2, 26, 1, 1, 14,
            1, 26, 29, 30, 1, 13, 21, 1, 1, 14, 21, 1, 23, 1, 15, 23, 21, 1, 30, 19, 19, 1, 10, 23,
            3, 3, 17, 22, 2, 26, 1, 11, 1, 23, 1, 1, 1, 15, 1, 1, 13, 1, 1
        ]),
        520317213
    );
}

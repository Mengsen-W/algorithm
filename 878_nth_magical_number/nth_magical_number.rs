/*
 * @Date: 2022-11-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-22
 * @FilePath: /algorithm/878_nth_magical_number/nth_magical_number.rs
 */

pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    fn gcd(first: usize, second: usize) -> usize {
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }

        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }

            max = min;
            min = res;
        }
    }

    const MOD: usize = 1000000007;
    let (n, a, b) = (n as usize, a as usize, b as usize);

    let c = a / gcd(a, b) * b;
    let m = c / a + c / b - 1;
    let r = n % m;
    let res = c * (n / m) % MOD;
    if r == 0 {
        return res as i32;
    }
    let mut add_a = a;
    let mut add_b = b;
    for _ in 0..(r - 1) {
        if add_a < add_b {
            add_a += a;
        } else {
            add_b += b;
        }
    }
    ((res + add_a.min(add_b) % MOD) % MOD) as i32
}

fn main() {
    assert_eq!(nth_magical_number(1, 2, 3), 2);
    assert_eq!(nth_magical_number(4, 2, 3), 6);
}

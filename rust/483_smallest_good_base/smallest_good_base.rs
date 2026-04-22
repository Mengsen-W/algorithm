/*
 * @Date: 2021-06-18 08:01:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-18 08:34:05
 */

fn smallest_good_base(n: String) -> String {
    let n_val: i64 = n.parse().unwrap();
    let m_max = ((n_val as f64).log2()).floor() as i32;
    for m in (2..=m_max).rev() {
        let k = (n_val as f64).powf(1.0_f64 / m as f64) as i64;
        let mut mul: i64 = 1;
        let mut sum: i64 = 1;
        for _i in 0..m {
            mul *= k;
            sum += mul;
        }
        if sum == n_val {
            return k.to_string();
        }
    }
    return (n_val - 1).to_string();
}

fn main() {
    assert_eq!(smallest_good_base("13".to_string()), "3".to_string());
    assert_eq!(smallest_good_base("4681".to_string()), "8".to_string());
    assert_eq!(
        smallest_good_base("1000000000000000000".to_string()),
        "999999999999999999".to_string()
    );
}

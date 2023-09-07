/*
 * @Date: 2023-09-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-07
 * @FilePath: /algorithm/rust/2549_repair_cars/repair_cars.rs
 */

struct Solution;
impl Solution {
    pub fn repair_cars(mut ranks: Vec<i32>, cars: i32) -> i64 {
        ranks.sort();
        let cars = cars as i64;
        let mut l = 1_i64;
        let mut r = 9223372036854775807_i64;
        while l < r {
            let mid = ((r - l) >> 1) + l;
            let mut top = false;
            let mut t = 0_i64;
            for v in &ranks {
                t += ((mid / (*v as i64)) as f64).sqrt() as i64;
                if t >= cars {
                    top = true;
                    break;
                }
            }
            if top {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}

fn main() {
    let tests = vec![(vec![4, 2, 3, 1], 10, 16), (vec![5, 1, 8], 6, 16)];
    for (ranks, cars, ans) in tests {
        assert_eq!(Solution::repair_cars(ranks, cars), ans);
    }
}

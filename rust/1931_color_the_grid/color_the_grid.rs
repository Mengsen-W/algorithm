struct Solution;

use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        // 哈希映射 valid 存储所有满足要求的对一行进行涂色的方案
        let mut valid = HashMap::new();
        // 在 [0, 3^m) 范围内枚举满足要求的 mask
        let mask_end = 3i32.pow(m as u32);
        for mask in 0..mask_end {
            let mut color = Vec::new();
            let mut mm = mask;
            for _ in 0..m {
                color.push(mm % 3);
                mm /= 3;
            }
            let mut check = true;
            for i in 0..m - 1 {
                if color[i] == color[i + 1] {
                    check = false;
                    break;
                }
            }
            if check {
                valid.insert(mask, color);
            }
        }

        // 预处理所有的 (mask1, mask2) 二元组，满足 mask1 和 mask2 作为相邻行时，同一列上两个格子的颜色不同
        let mut adjacent = HashMap::new();
        for (&mask1, color1) in &valid {
            for (&mask2, color2) in &valid {
                let mut check = true;
                for i in 0..m {
                    if color1[i] == color2[i] {
                        check = false;
                        break;
                    }
                }
                if check {
                    adjacent.entry(mask1).or_insert(Vec::new()).push(mask2);
                }
            }
        }

        let mut f = HashMap::new();
        for &mask in valid.keys() {
            f.insert(mask, 1);
        }
        for _ in 1..n {
            let mut g = HashMap::new();
            for &mask2 in valid.keys() {
                let mut total = 0;
                if let Some(list) = adjacent.get(&mask2) {
                    for &mask1 in list {
                        total = (total + f.get(&mask1).unwrap_or(&0)) % MOD;
                    }
                }
                g.insert(mask2, total);
            }
            f = g;
        }

        let mut ans = 0;
        for &num in f.values() {
            ans = (ans + num) % MOD;
        }
        ans
    }
}

fn main() {
    let tests = vec![(1, 1, 3), (1, 2, 6), (5, 5, 580986)];

    for (n, m, expected) in tests {
        assert_eq!(Solution::color_the_grid(n, m), expected);
    }
}

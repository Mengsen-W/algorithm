/*
 * @Date: 2023-05-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-19
 * @FilePath: /algorithm/rust/1079_num_tile_possibilities/num_tile_possibilities.rs
 */

struct Solution;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut t = vec![0; 26];

        for b in tiles.bytes() {
            t[b as usize - 'A' as usize] += 1;
        }

        Self::dfs(&mut t)
    }

    fn dfs(t: &mut [i32]) -> i32 {
        let mut res = 0;

        for i in 0..26 {
            if t[i] > 0 {
                t[i] -= 1;
                res += Self::dfs(t) + 1;
                t[i] += 1;
            }
        }

        res
    }
}

fn main() {
    {
        let tiles = "AAB".to_string();
        let ans = 8;
        assert_eq!(Solution::num_tile_possibilities(tiles), ans);
    }

    {
        let tiles = "AAABBC".to_string();
        let ans = 188;
        assert_eq!(Solution::num_tile_possibilities(tiles), ans);
    }

    {
        let tiles = "V".to_string();
        let ans = 1;
        assert_eq!(Solution::num_tile_possibilities(tiles), ans);
    }
}

/*
 * @Date: 2023-03-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-09
 * @FilePath: /algorithm/rust/2379_minimum_recolors/minimum_recolors.rs
 */

pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let (mut b_cnt, mut ret, block_arr) = (0, k, blocks.as_bytes());
    for i in 0..block_arr.len() {
        if block_arr[i] == b'B' {
            b_cnt += 1;
        }
        if i as i32 >= k && block_arr[i - k as usize] == b'B' {
            b_cnt -= 1;
        }
        ret = ret.min(k - b_cnt);
    }
    ret
}

fn main() {
    {
        let blocks = "WBBWWBBWBW".to_string();
        let k = 7;
        let ans = 3;
        assert_eq!(minimum_recolors(blocks, k), ans);
    }

    {
        let blocks = "WBWBBBW".to_string();
        let k = 2;
        let ans = 0;
        assert_eq!(minimum_recolors(blocks, k), ans);
    }
}

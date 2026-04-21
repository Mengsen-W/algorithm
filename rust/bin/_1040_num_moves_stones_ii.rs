/*
 * @Date: 2023-04-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-07
 * @FilePath: /algorithm/rust/1040_num_moves_stones_ii/num_moves_stones_ii.rs
 */

pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
    let mut stones = stones;
    stones.sort();
    let n = stones.len();
    let max = std::cmp::max(
        stones[n - 1] - stones[1] - n as i32 + 2,
        stones[n - 2] - stones[0] - n as i32 + 2,
    );
    let mut min = max;
    let mut i = 0;
    let mut j = 0;
    while j < n {
        while stones[j] - stones[i] + 1 > n as i32 {
            i += 1;
        }
        if j - i + 1 == n - 1 && stones[j] - stones[i] + 1 == n as i32 - 1 {
            min = std::cmp::min(min, 2);
        } else {
            min = std::cmp::min(min, n as i32 - (j - i + 1) as i32);
        }
        j += 1;
    }
    vec![min, max]
}

fn main() {
    {
        let stones = vec![7, 4, 9];
        let ans = vec![1, 2];
        assert_eq!(num_moves_stones_ii(stones), ans);
    }

    {
        let stones = vec![6, 5, 4, 3, 10];
        let ans = vec![2, 3];
        assert_eq!(num_moves_stones_ii(stones), ans);
    }

    {
        let stones = vec![100, 101, 104, 102, 103];
        let ans = vec![0, 0];
        assert_eq!(num_moves_stones_ii(stones), ans);
    }
}

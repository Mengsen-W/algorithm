/*
 * @Date: 2023-04-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-30
 * @FilePath: /algorithm/rust/1033_num_moves_stones/num_moves_stones.rs
 */

pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
    let mut places: Vec<i32> = vec![a, b, c];
    places.sort();
    let max_num: i32 = places[2] - places[0] - 2;
    let min_num: i32;
    let mut gaps: Vec<i32> = vec![places[1] - places[0] - 1, places[2] - places[1] - 1];
    gaps.sort();
    min_num = if gaps[0] == 0 {
        if gaps[1] == 0 {
            0
        } else {
            1
        }
    } else if gaps[0] == 1 {
        1
    } else {
        2
    };
    vec![min_num, max_num]
}

fn main() {
    {
        let a = 1;
        let b = 2;
        let c = 5;
        let ans = vec![1, 2];
        assert_eq!(num_moves_stones(a, b, c), ans);
    }

    {
        let a = 4;
        let b = 3;
        let c = 2;
        let ans = vec![0, 0];
        assert_eq!(num_moves_stones(a, b, c), ans);
    }
}

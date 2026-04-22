/*
 * @Date: 2022-01-19 16:15:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-19 16:30:33
 */

pub fn stone_game_ix(stones: Vec<i32>) -> bool {
    let (mut cnt_0, mut cnt_1, mut cnt_2) = (0, 0, 0);
    for val in stones {
        let type_ = val % 3;
        if type_ == 0 {
            cnt_0 += 1;
        } else if type_ == 1 {
            cnt_1 += 1;
        } else {
            cnt_2 += 1
        }
    }

    if cnt_0 % 2 == 0 {
        cnt_1 >= 1 && cnt_2 >= 1
    } else {
        cnt_1 - cnt_2 > 2 || cnt_2 - cnt_1 > 2
    }
}

fn main() {
    assert_eq!(stone_game_ix(vec![2, 1]), true);
    assert_eq!(stone_game_ix(vec![2]), false);
    assert_eq!(stone_game_ix(vec![5, 1, 2, 4, 3]), false);
}

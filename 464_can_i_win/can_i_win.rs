/*
 * @Date: 2022-05-22 09:50:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-22 09:57:44
 * @FilePath: /algorithm/464_can_i_win/can_i_win.rs
 */

pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    fn dfs(cur: i32, range: i32, target: i32, f: &mut Vec<Option<bool>>) -> bool {
        let ucur = cur as usize;
        if let Some(ok) = f[ucur] {
            return ok;
        }
        for i in (1..=range).rev() {
            let cur_bit = 1 << (i - 1);
            if cur & cur_bit == 0 {
                if i >= target || !dfs(cur | cur_bit, range, target - i, f) {
                    f[ucur] = Some(true);
                    return true;
                }
            }
        }
        f[ucur] = Some(false);
        false
    }
    let can_reach = (max_choosable_integer + 1) * max_choosable_integer / 2;
    if can_reach < desired_total {
        return false;
    }
    if can_reach == desired_total {
        return max_choosable_integer & 1 == 1;
    }
    let mut f = vec![None; 1 << (max_choosable_integer as usize)];
    dfs(0, max_choosable_integer, desired_total, &mut f)
}

fn main() {
    assert_eq!(can_i_win(10, 11), false);
    assert_eq!(can_i_win(10, 0), true);
    assert_eq!(can_i_win(10, 1), true);
}

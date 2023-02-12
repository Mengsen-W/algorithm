/*
 * @Date: 2023-02-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-12
 * @FilePath: /algorithm/rust/1138_alphabet_board_path/alphabet_board_path.rs
 */

pub fn alphabet_board_path(target: String) -> String {
    let mut res: String = String::new();
    let mut char_places: Vec<(i32, i32)> = vec![(0, 0); 26];
    for row in 0..5 {
        for col in 0..5 {
            char_places[row * 5 + col].0 = row as i32;
            char_places[row * 5 + col].1 = col as i32;
        }
    }
    char_places[25].0 = 5;
    char_places[25].1 = 0;
    let (mut x, mut y): (i32, i32) = (0_i32, 0_i32);
    for target_byte in target.bytes() {
        let (target_x, target_y) = char_places[target_byte as usize - 97];
        if x == 5 && y == 0 {
            if target_x > x {
                res.push_str("D".repeat((target_x - x) as usize).as_str());
            } else {
                res.push_str("U".repeat((x - target_x) as usize).as_str());
            }
            if target_y > y {
                res.push_str("R".repeat((target_y - y) as usize).as_str());
            } else {
                res.push_str("L".repeat((y - target_y) as usize).as_str());
            }
        } else {
            if target_y > y {
                res.push_str("R".repeat((target_y - y) as usize).as_str());
            } else {
                res.push_str("L".repeat((y - target_y) as usize).as_str());
            }
            if target_x > x {
                res.push_str("D".repeat((target_x - x) as usize).as_str());
            } else {
                res.push_str("U".repeat((x - target_x) as usize).as_str());
            }
        }
        res.push('!');
        x = target_x;
        y = target_y;
    }
    res
}

fn main() {
    {
        let target = String::from("leet");
        let ans = "RDD!RRRUU!!DDD!".to_string();
        assert_eq!(alphabet_board_path(target), ans);
    }

    {
        let target = "code".to_string();
        let ans = "RR!RRDD!LUU!R!".to_string();
        assert_eq!(alphabet_board_path(target), ans);
    }
}

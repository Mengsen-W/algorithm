/*
 * @Date: 2022-03-22 00:45:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-22 01:20:54
 * @FilePath: /algorithm/2038_winner_of_game/winner_of_game.rs
 */

pub fn winner_of_game(colors: String) -> bool {
    let mut freq: [i32; 2] = [0, 0];
    let mut cur = 'c';
    let mut cnt = 0;
    for c in colors.chars() {
        if c != cur {
            cur = c;
            cnt = 1;
        } else {
            cnt += 1;
            if cnt >= 3 {
                freq[(cur as u8 - 'A' as u8) as usize] += 1;
            }
        }
    }
    freq[0] > freq[1]
}

fn main() {
    assert_eq!(winner_of_game("AAABABB".to_string()), true);
    assert_eq!(winner_of_game("AA".to_string()), false);
    assert_eq!(winner_of_game("ABBBBBBBAAA".to_string()), false);
}

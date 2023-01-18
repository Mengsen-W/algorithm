/*
 * @Date: 2021-11-09 01:24:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-09 02:18:54
 */

struct Solution;

// impl Solution {
//     pub fn find_min_step(mut board: String, hand: String) -> i32 {
//         const MAX_COUNT: i32 = 6;
//         fn remove_consecutive(s: &str) -> String {
//             let mut i = 0;
//             for j in 0..s.len() {
//                 if s.as_bytes()[j] == s.as_bytes()[i] {
//                     continue;
//                 }
//                 if j - i >= 3 {
//                     let mut s2 = String::from(&s[..i]);
//                     s2.push_str(&s[j..]);
//                     return remove_consecutive(&s2);
//                 } else {
//                     i = j;
//                 }
//             }
//             s.to_owned()
//         }

//         fn dfs(str: &str, h: &mut Vec<i32>, remove: bool) -> i32 {
//             let s = remove_consecutive(str);
//             let s = if remove { &s } else { str };
//             if s == "#" {
//                 return 0;
//             }
//             let mut ans = MAX_COUNT;
//             let mut i = 0;
//             for j in 0..s.len() {
//                 if s.as_bytes()[j] == s.as_bytes()[i] {
//                     continue;
//                 }
//                 let needed = 3 - (j - i) as i32;
//                 if needed >= 0 && h[(s.as_bytes()[i] - b'A') as usize] >= needed {
//                     h[(s.as_bytes()[i] - b'A') as usize] -= needed;
//                     let mut s2 = String::from(&s[..i]);
//                     s2.push_str(&s[j..]);
//                     ans = std::cmp::min(ans, needed + dfs(&s2, h, true));
//                     ans = std::cmp::min(ans, needed + dfs(&s2, h, false));
//                     h[(s.as_bytes()[i] - b'A') as usize] += needed;
//                 }
//                 i = j;
//             }
//             ans
//         }
//         let mut hand_count = vec![0; 26];
//         for i in 0..hand.len() {
//             hand_count[(hand.as_bytes()[i] - b'A') as usize] += 1;
//         }
//         board.push('#');
//         let step = dfs(&board, &mut hand_count, true);
//         println!("{}", step);
//         if step == MAX_COUNT {
//             -1
//         } else {
//             step
//         }
//     }
// }

use std::ops::Range;

impl Solution {
    fn find_min_step(board: String, hand: String) -> i32 {
        let n = hand.len();
        let board: Vec<char> = board.chars().collect();
        let hand: Vec<char> = hand.chars().collect();
        let mut res = std::i32::MAX;
        Self::dfs(0, 0, board, &mut res, &hand, n);
        if res == std::i32::MAX {
            -1
        } else {
            res
        }
    }

    fn dfs(start: usize, state: u32, board: Vec<char>, res: &mut i32, hand: &[char], n: usize) {
        if start == n {
            return;
        }
        for i in 0..board.len() {
            if i == 0 || board[i] != board[i - 1] {
                if let Some(next_state) = Self::find_next_state(board[i], state, hand, n) {
                    let mut new_board: Vec<char> = board.to_vec();
                    new_board.insert(i, board[i]);
                    while let Some(range) = Self::dropable(&new_board) {
                        new_board.drain(range);
                        Self::dfs(start + 1, next_state, new_board.to_vec(), res, hand, n);
                    }
                    if new_board.is_empty() {
                        *res = (*res).min((start + 1) as i32);
                    } else {
                        Self::dfs(start + 1, next_state, new_board, res, hand, n);
                    }
                }
            }
        }
    }

    fn find_next_state(c: char, state: u32, hand: &[char], n: usize) -> Option<u32> {
        for i in 0..n {
            if hand[i] == c && state & 1 << i == 0 {
                return Some(state | 1 << i);
            }
        }
        None
    }

    fn dropable(board: &[char]) -> Option<Range<usize>> {
        let n = board.len();
        let mut l = 0;
        let mut r = 0;
        while r < n {
            if board[l] == board[r] {
                r += 1;
            } else {
                if r - l >= 3 {
                    return Some(l..r);
                } else {
                    l = r;
                }
            }
        }
        if r - l >= 3 {
            return Some(l..r);
        }
        None
    }
}

fn main() {
    // assert_eq!(
    //     Solution::find_min_step("WRRBBW".to_string(), "RB".to_string()),
    //     -1
    // );
    // assert_eq!(
    //     Solution::find_min_step("WWRRBBWW".to_string(), "WRBRW".to_string()),
    //     2
    // );
    // assert_eq!(
    //     Solution::find_min_step("G".to_string(), "GGGGG".to_string()),
    //     2
    // );
    // assert_eq!(
    //     Solution::find_min_step("RBYYBBRRB".to_string(), "YRBGB".to_string()),
    //     3
    // );
    assert_eq!(
        Solution::find_min_step("WWBBWBBWW".to_string(), "BB".to_string()),
        -1
    );
}

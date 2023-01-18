/*
 * @Date: 2021-09-16 09:05:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-16 09:17:03
 */

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

struct Trie {
    next: Vec<Option<Rc<RefCell<Trie>>>>,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            next: vec![None; 26],
        }
    }

    fn insert(head: Rc<RefCell<Trie>>, word: &str) {
        let mut h = head;
        for c in word.chars() {
            if h.borrow().next[c as usize - 'a' as usize].is_none() {
                let new = Rc::new(RefCell::new(Trie::new()));
                h.borrow_mut().next[c as usize - 'a' as usize] = Some(Rc::clone(&new));
            }
            let tmp = Rc::clone(h.borrow().next[c as usize - 'a' as usize].as_ref().unwrap());
            h = tmp;
        }
    }

    fn find_word(head: Rc<RefCell<Trie>>, word: &str) -> bool {
        let mut h = head;
        for c in word.chars() {
            if h.borrow().next[c as usize - 'a' as usize].is_none() {
                return false;
            }
            let tmp = Rc::clone(h.borrow().next[c as usize - 'a' as usize].as_ref().unwrap());
            h = tmp;
        }
        true
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let trie = Rc::new(RefCell::new(Trie::new()));
        Solution::build_trie(Rc::clone(&trie), &board);
        words
            .into_iter()
            .filter(|x| Trie::find_word(Rc::clone(&trie), x))
            .collect()
    }

    fn build_trie(trie: Rc<RefCell<Trie>>, board: &Vec<Vec<char>>) {
        let (row_num, col_num) = (board.len(), board[0].len());
        let deep = if row_num * col_num > 10 {
            10
        } else {
            (row_num * col_num) as i32
        };
        for i in 0..row_num {
            for j in 0..col_num {
                let mut visited = vec![vec![0; col_num]; row_num];
                Solution::dfs(
                    board,
                    deep,
                    row_num,
                    col_num,
                    &mut visited,
                    i,
                    j,
                    0,
                    &mut String::new(),
                    Rc::clone(&trie),
                );
            }
        }
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        deep: i32,
        row_num: usize,
        col_num: usize,
        visited: &mut Vec<Vec<u8>>,
        r: usize,
        c: usize,
        steps: i32,
        frame: &mut String,
        trie: Rc<RefCell<Trie>>,
    ) {
        if steps == deep {
            return;
        }

        if visited[r][c] == 1 {
            return;
        }

        frame.push(board[r][c]);
        visited[r][c] = 1;

        if r as i32 - 1 >= 0 {
            Solution::dfs(
                board,
                deep,
                row_num,
                col_num,
                visited,
                r - 1,
                c,
                steps + 1,
                frame,
                Rc::clone(&trie),
            );
        }
        if r + 1 < row_num {
            Solution::dfs(
                board,
                deep,
                row_num,
                col_num,
                visited,
                r + 1,
                c,
                steps + 1,
                frame,
                Rc::clone(&trie),
            );
        }
        if c as i32 - 1 >= 0 {
            Solution::dfs(
                board,
                deep,
                row_num,
                col_num,
                visited,
                r,
                c - 1,
                steps + 1,
                frame,
                Rc::clone(&trie),
            );
        }
        if c + 1 < col_num {
            Solution::dfs(
                board,
                deep,
                row_num,
                col_num,
                visited,
                r,
                c + 1,
                steps + 1,
                frame,
                Rc::clone(&trie),
            );
        }

        Trie::insert(trie, &frame);

        frame.pop();
        visited[r][c] = 0;
    }
}

fn main() {
    {
        let board: Vec<Vec<char>> = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words: Vec<String> = vec!["oath", "pea", "eat", "rain"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let ans: Vec<String> = vec!["oath", "eat"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::find_words(board, words), ans);
    }
    {
        let board: Vec<Vec<char>> = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words: Vec<String> = vec!["abcd"].iter().map(|x| x.to_string()).collect();
        let ans: Vec<String> = vec![];
        assert_eq!(Solution::find_words(board, words), ans);
    }
}

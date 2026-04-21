/*
 * @Date: 2022-06-27
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-27
 * @FilePath: /algorithm/522_find_lu_slength/find_lu_slength.rs
 */

struct Solution;

impl Solution {
    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        strs.sort_by(|x, y| (y.len()).cmp(&x.len()));

        let l: usize = strs.len();

        for i in 0..l {
            let mut b = true;

            for j in 0..l {
                if i != j && Self::is_sub_seq(&strs[i], &strs[j]) {
                    b = false;
                    break;
                }
            }

            if b {
                return strs[i].len() as i32;
            }
        }

        -1
    }

    fn is_sub_seq(x: &String, y: &String) -> bool {
        let sx = x.as_bytes();
        let sy = y.as_bytes();
        let n = sx.len();
        let m = sy.len();
        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            if sx[i] == sy[j] {
                i += 1;
            }

            j += 1;
        }

        i == n
    }
}

fn main() {
    {
        let strs = vec![
            String::from("aba"),
            String::from("cdc"),
            String::from("eae"),
        ];
        assert_eq!(Solution::find_lu_slength(strs), 3);
    }

    {
        let strs = vec![String::from("aaa"), String::from("aaa"), String::from("aa")];
        assert_eq!(Solution::find_lu_slength(strs), -1);
    }
}

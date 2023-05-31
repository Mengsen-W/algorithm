/*
 * @Date: 2023-05-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-31
 * @FilePath: /algorithm/rust/1130_mct_from_leaf_values/mct_from_leaf_values.rs
 */

pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stk = vec![arr[0]];
    for &n in arr.iter().skip(1) {
        let mut cur = n;
        while !stk.is_empty() && *stk.last().unwrap() <= cur {
            let top = stk.pop().unwrap();
            if stk.is_empty() || *stk.last().unwrap() > cur {
                cur = cur.max(top);
                ans += cur * top;
            } else {
                ans += top * stk.last().unwrap();
            }
        }
        stk.push(cur);
    }
    stk.windows(2).rev().for_each(|x| {
        ans += x[0] * x[1];
    });
    ans
}

fn main() {
    {
        let arr = vec![6, 2, 4];
        let ans = 32;
        assert_eq!(mct_from_leaf_values(arr), ans);
    }

    {
        let arr = vec![4, 11];
        let ans = 44;
        assert_eq!(mct_from_leaf_values(arr), ans);
    }
}

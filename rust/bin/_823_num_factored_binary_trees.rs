/*
 * @Date: 2023-08-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-29
 * @FilePath: /algorithm/rust/823_num_factored_binary_trees/num_factored_binary_trees.rs
 */

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort_unstable();
        let len = arr.len();
        let mut ans: i64 = 1;
        let mut map: HashMap<i32, i64> = arr.iter().map(|a| (*a, 1)).into_iter().collect();
        let mod_num: i64 = 1_000_000_007;
        for i in 1..len {
            let cur = arr[i];
            let mut j = 0;
            let mut count = 1;
            while arr[j] * arr[j] < cur {
                if cur % arr[j] == 0 && map.contains_key(&(cur / arr[j])) {
                    count += (map.get(&arr[j]).unwrap() * map.get(&(cur / arr[j])).unwrap()) << 1;
                    count %= mod_num;
                }
                j += 1;
            }
            if arr[j] * arr[j] == cur {
                let val = map.get(&arr[j]).unwrap();
                count += val * val;
            }
            map.insert(cur, count);
            ans = (ans + count) % mod_num;
        }
        // print!("{:?}", map);

        return ans as i32;
    }
}

fn main() {
    let tests = vec![(vec![2, 4], 3), (vec![2, 4, 5, 10], 7)];

    for (arr, ans) in tests {
        assert_eq!(Solution::num_factored_binary_trees(arr), ans)
    }
}

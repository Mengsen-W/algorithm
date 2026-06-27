struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut cnt: HashMap<i64, i32> = HashMap::new();

        for num in nums {
            *cnt.entry(num as i64).or_insert(0) += 1;
        }

        // ans 至少是 1 的数量，向下取奇数
        let one_cnt = *cnt.get(&1).unwrap_or(&0);
        let mut ans = if one_cnt % 2 == 1 {
            one_cnt
        } else {
            one_cnt - 1
        };

        cnt.remove(&1);

        for &num in cnt.keys() {
            let mut res = 0;
            let mut x = num;

            while matches!(cnt.get(&x), Some(&c) if c > 1) {
                res += 2;
                x *= x;
            }

            ans = ans.max(res + if cnt.contains_key(&x) { 1 } else { -1 });
        }

        ans
    }
}

fn main() {
    let tests = vec![(vec![5, 4, 1, 2, 2], 3), (vec![1, 3, 2, 4], 1)];

    for (nums, expected) in tests {
        assert_eq!(Solution::maximum_length(nums), expected);
    }
}

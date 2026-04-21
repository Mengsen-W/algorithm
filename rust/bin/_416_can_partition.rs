struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let s = nums.iter().sum::<i32>();
        if s % 2 != 0 {
            return false;
        }
        let s = s as usize / 2; // 注意这里把 s 减半了
        let mut f = vec![false; s + 1];
        f[0] = true;
        let mut s2 = 0;
        for x in nums {
            let x = x as usize;
            s2 = (s2 + x).min(s);
            for j in (x..=s2).rev() {
                f[j] = f[j] || f[j - x];
            }
            if f[s] {
                return true;
            }
        }
        false
    }
}

fn main() {
    let tests = vec![(vec![1, 5, 11, 5], true), (vec![1, 2, 3, 5], false)];

    for (nums, ans) in tests {
        assert_eq!(Solution::can_partition(nums), ans);
    }
}

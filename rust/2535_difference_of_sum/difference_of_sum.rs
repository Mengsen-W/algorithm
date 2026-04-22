struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for mut x in nums {
            ans += x; // 累加元素和
            while x > 0 {
                ans -= x % 10; // 减去数位和
                x /= 10;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 15, 6, 3], 9), (vec![1, 2, 3, 4], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::difference_of_sum(nums), ans);
    }
}

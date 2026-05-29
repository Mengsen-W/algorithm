struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut ans = 37;
        for num in nums {
            let mut dig = 0;
            let mut n = num;
            while n > 0 {
                dig += n % 10;
                n /= 10;
            }
            ans = ans.min(dig);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![10, 12, 13, 14], 1),
        (vec![1, 2, 3, 4], 1),
        (vec![999, 19, 199], 10),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::min_element(nums), expected);
    }
}

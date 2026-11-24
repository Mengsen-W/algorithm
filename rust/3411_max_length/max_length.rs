struct Solution;

impl Solution {
    fn max_length(nums: Vec<i32>) -> i32 {
        let mut res = 2;
        let mut prod_val = 1;
        let n = nums.len();
        let mut start = 0;
        let mut end = 0;

        let gcd = |mut a: i32, mut b: i32| -> i32 {
            while b != 0 {
                let temp = a % b;
                a = b;
                b = temp;
            }
            a
        };

        while end < n {
            while gcd(prod_val, nums[end]) > 1 {
                prod_val /= nums[start];
                start += 1;
            }
            prod_val *= nums[end];
            res = res.max((end - start + 1) as i32);
            end += 1;
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 1, 2, 1, 1, 1], 5),
        (vec![2, 3, 4, 5, 6], 3),
        (vec![1, 2, 3, 1, 4, 5, 1], 5),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::max_length(nums), expected);
    }
}

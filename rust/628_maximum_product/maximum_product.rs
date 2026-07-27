struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut min1 = i32::MAX;
        let mut min2 = i32::MAX;
        let mut max1 = i32::MIN;
        let mut max2 = i32::MIN;
        let mut max3 = i32::MIN;

        for x in nums {
            // 维护最小值和次小值
            if x < min1 {
                min2 = min1;
                min1 = x;
            } else if x < min2 {
                min2 = x;
            }

            // 维护前三大
            if x > max1 {
                max3 = max2;
                max2 = max1;
                max1 = x;
            } else if x > max2 {
                max3 = max2;
                max2 = x;
            } else if x > max3 {
                max3 = x;
            }
        }

        (max1 * max2 * max3).max(min1 * min2 * max1)
    }
}

fn main() {
    let tests = vec![
        ( vec![ 1, 2, 3 ], 6 ),
        ( vec![ 1, 2, 3, 4 ], 24 ),
        ( vec![ -1, -2, -3 ], -6 ),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::maximum_product(nums), expected);
    }
}

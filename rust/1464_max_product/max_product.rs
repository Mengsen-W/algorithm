struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut first, mut second) = (0, 0);
        nums.iter().for_each(|&x| {
            if x > first {
                second = first;
                first = x;
            } else if x > second {
                second = x;
            }
        });
        (first - 1) * (second - 1)
    }
} 

fn main() {
    let tests = vec![
        ( vec![ 3, 4, 5, 2 ], 12 ),
        ( vec![ 1, 5, 4, 5 ], 16 ),
        ( vec![ 3, 7 ], 12 ),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::max_product(nums), expected);
    }
}

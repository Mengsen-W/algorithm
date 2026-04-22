struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for &num in &nums {
            let digits = (num as f64).log10().floor() as i32 + 1;
            if digits % 2 == 0 {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![12, 345, 2, 6, 7896], 2),
        (vec![555, 901, 482, 1771], 1),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::find_numbers(nums), ans);
    }
}

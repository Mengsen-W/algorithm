struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let area = (right - left) as i32 * height[left].min(height[right]);
            ans = ans.max(area);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49), (vec![1, 1], 1)];

    for (height, ans) in tests {
        assert_eq!(Solution::max_area(height), ans);
    }
}

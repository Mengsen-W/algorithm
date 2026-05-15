struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let pivot = low + (high - low) / 2;
            if nums[pivot] < nums[high] {
                high = pivot;
            } else {
                low = pivot + 1
            }
        }
        nums[low]
    }
}

fn main() {
    let tests = vec![
        (vec![3, 4, 5, 1, 2], 1),
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![11, 13, 15, 17], 11),
    ];
    for (nums, ans) in tests {
        assert_eq!(Solution::find_min(nums), ans);
    }
}

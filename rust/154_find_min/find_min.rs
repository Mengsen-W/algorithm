struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let pivot = low + (high - low) / 2;
            if nums[pivot] < nums[high] {
                high = pivot;
            } else if nums[pivot] > nums[high] {
                low = pivot + 1;
            } else {
                high -= 1;
            }
        }
        return nums[low];
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 5], 1), (vec![2, 2, 2, 0, 1], 0)];

    for (nums, expected) in tests {
        assert_eq!(Solution::find_min(nums), expected);
    }
}

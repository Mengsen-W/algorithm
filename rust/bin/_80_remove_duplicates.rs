struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let size = nums.len();
        if size <= 2 {
            return size as i32;
        }
        let mut slow = 2;
        let mut fast = 2;
        while fast < size {
            if nums[slow - 2] != nums[fast] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1, 1, 1, 2, 2, 3], 5),
        (vec![0, 0, 1, 1, 1, 1, 2, 3, 3], 7),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::remove_duplicates(nums), ans);
    }
}

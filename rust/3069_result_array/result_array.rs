struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr_1 = vec![nums[0]];
        let mut arr_2 = vec![nums[1]];
        for i in 2..nums.len() {
            if arr_1.last().unwrap() > arr_2.last().unwrap() {
                arr_1.push(nums[i]);
            } else {
                arr_2.push(nums[i]);
            }
        }
        arr_1.extend(arr_2);
        arr_1
    }
}

fn main() {
    let tests = vec![
        (vec![2, 1, 3], vec![2, 3, 1]),
        (vec![5, 4, 3, 8], vec![5, 3, 4, 8]),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::result_array(nums), expected);
    }
}

struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];

        // 将元素值与原下标绑定
        let mut arr: Vec<(i32, usize)> =
            nums.into_iter().enumerate().map(|(i, x)| (x, i)).collect();

        // 按元素值升序排序
        arr.sort_by_key(|p| p.0);

        let values: Vec<i32> = arr.iter().map(|p| p.0).collect();
        let indices: Vec<usize> = arr.iter().map(|p| p.1).collect();

        let mut i = 0;
        while i < n {
            let start = i;

            // 当前连通块中的原下标
            let mut groupIndices = Vec::new();

            // 当前连通块中的元素值
            let mut groupValues = Vec::new();

            while i < n && (i == start || values[i] - values[i - 1] <= limit) {
                groupIndices.push(indices[i]);
                groupValues.push(values[i]);
                i += 1;
            }

            // 由于元素值数组已经有序，这里不需要再排序
            groupIndices.sort();

            // 为得到字典序最小的结果，将较小元素放到较小下标处
            for (index, value) in groupIndices.into_iter().zip(groupValues.into_iter()) {
                ans[index] = value;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 5, 3, 9, 8], 2, vec![1, 3, 5, 8, 9]),
        (vec![1, 7, 6, 18, 2, 1], 3, vec![1, 6, 7, 18, 1, 2]),
        (vec![1, 7, 28, 19, 10], 3, vec![1, 7, 28, 19, 10]),
    ];

    for (nums, limit, expected) in tests {
        assert_eq!(
            Solution::lexicographically_smallest_array(nums, limit),
            expected
        );
    }
}

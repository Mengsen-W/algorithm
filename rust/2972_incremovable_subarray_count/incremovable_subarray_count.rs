struct Solution;

impl Solution {
    pub fn incremovable_subarray_count(a: Vec<i32>) -> i64 {
        let n = a.len();
        let mut i = 0;
        while i < n - 1 && a[i] < a[i + 1] {
            i += 1;
        }
        if i == n - 1 {
            // 每个非空子数组都可以移除
            return n as i64 * (n + 1) as i64 / 2;
        }

        let mut i = i as i64;
        let mut ans = i + 2; // 不保留后缀的情况，一共 i+2 个
                             // 枚举保留的后缀为 a[j:]
        let mut j = n - 1;
        while j == n - 1 || a[j] < a[j + 1] {
            while i >= 0 && a[i as usize] >= a[j] {
                i -= 1;
            }
            // 可以保留前缀 a[:i+1], a[:i], ..., a[:0] 一共 i+2 个
            ans += i + 2;
            j -= 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4], 10),
        (vec![6, 5, 7, 8], 7),
        (vec![8, 7, 6, 6], 3),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::incremovable_subarray_count(nums), ans);
    }
}

struct Solution;

impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut res = 0; // 相减操作的总次数
        while num1 != 0 && num2 != 0 {
            // 每一步辗转相除操作
            res += num1 / num2;
            num1 %= num2;
            // 交换两个数
            std::mem::swap(&mut num1, &mut num2);
        }
        res
    }
}

fn main() {
    let tests = vec![(2, 3, 3), (10, 10, 1)];

    for (num1, num2, ans) in tests {
        assert_eq!(Solution::count_operations(num1, num2), ans);
    }
}

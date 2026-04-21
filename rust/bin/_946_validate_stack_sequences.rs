/*
 * @Date: 2022-08-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-31
 * @FilePath: /algorithm/946_validate_stack_sequences/validate_stack_sequences.rs
 */

pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut stack = vec![];
    let mut out = 0;

    pushed.iter().for_each(|&v| {
        stack.push(v);
        while let Some(&n) = stack.last() {
            if popped[out] != n {
                break;
            }
            stack.pop();
            out += 1;
        }
    });
    stack.is_empty()
}

fn main() {
    {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        assert!(validate_stack_sequences(pushed, popped));
    }
    {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        assert!(!validate_stack_sequences(pushed, popped));
    }
}

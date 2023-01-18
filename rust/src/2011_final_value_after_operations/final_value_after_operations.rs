/*
 * @Date: 2022-12-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-23
 * @FilePath: /algorithm/2011_final_value_after_operations/final_value_after_operations.rs
 */

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.iter().fold(0, |mut x: i32, op: &String| {
        match op.chars().nth(1).unwrap() {
            '-' => x -= 1,
            _ => x += 1,
        }
        x
    })
}

fn main() {
    {
        let operations = vec!["--X", "X++", "X++"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 1;
        assert_eq!(final_value_after_operations(operations), ans);
    }

    {
        let operations = vec!["++X", "++X", "X++"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 3;
        assert_eq!(final_value_after_operations(operations), ans);
    }

    {
        let operations = vec!["X++", "++X", "--X", "X--"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 0;
        assert_eq!(final_value_after_operations(operations), ans);
    }
}

/*
 * @Date: 2022-10-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-19
 * @FilePath: /algorithm/1700_count_students/count_students.rs
 */

pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut s1: i32 = students.iter().sum();
    let mut s0: i32 = students.len() as i32 - s1;
    for i in 0..sandwiches.len() {
        if sandwiches[i] == 0 && s0 > 0 {
            s0 -= 1;
        } else if sandwiches[i] == 1 && s1 > 0 {
            s1 -= 1;
        } else {
            break;
        }
    }
    s0 + s1
}

fn main() {
    {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];
        let ans = 0;
        assert_eq!(count_students(students, sandwiches), ans);
    }

    {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];
        let ans = 3;
        assert_eq!(count_students(students, sandwiches), ans);
    }
}

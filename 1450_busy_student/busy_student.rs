/*
 * @Date: 2022-08-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-19
 * @FilePath: /algorithm/1450_busy_student/busy_student.rs
 */

pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    start_time
        .iter()
        .zip(end_time.iter())
        .filter(|(&start, &end)| query_time >= start && query_time <= end)
        .count() as i32
}

fn main() {
    {
        let start_time = vec![1, 2, 3];
        let end_time = vec![3, 2, 7];
        let query_time = 4;
        assert_eq!(busy_student(start_time, end_time, query_time), 1);
    }

    {
        let start_time = vec![4];
        let end_time = vec![4];
        let query_time = 4;
        assert_eq!(busy_student(start_time, end_time, query_time), 1);
    }

    {
        let start_time = vec![1, 1, 1, 1];
        let end_time = vec![1, 3, 2, 4];
        let query_time = 7;
        assert_eq!(busy_student(start_time, end_time, query_time), 0);
    }

    {
        let start_time = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let end_time = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
        let query_time = 5;
        assert_eq!(busy_student(start_time, end_time, query_time), 5);
    }

    {
        let start_time = vec![1, 2, 3];
        let end_time = vec![3, 2, 7];
        let query_time = 4;
        assert_eq!(busy_student(start_time, end_time, query_time), 1);
    }
}

/*
 * @Date: 2022-12-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-31
 * @FilePath: /algorithm/2037_min_moves_to_seat/min_moves_to_seat.rs
 */

pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let (mut seats, mut students) = (seats, students);
    seats.sort();
    students.sort();
    seats
        .iter()
        .zip(students.iter())
        .fold(0, |cnt, (i, j)| cnt + (i - j).abs())
}

fn main() {
    {
        let seats = vec![3, 1, 5];
        let students = vec![2, 7, 4];
        let ans = 4;
        assert_eq!(min_moves_to_seat(seats, students), ans);
    }

    {
        let seats = vec![4, 1, 5, 9];
        let students = vec![1, 3, 2, 6];
        let ans = 7;
        assert_eq!(min_moves_to_seat(seats, students), ans);
    }

    {
        let seats = vec![2, 2, 6, 6];
        let students = vec![1, 3, 2, 6];
        let ans = 4;
        assert_eq!(min_moves_to_seat(seats, students), ans);
    }
}

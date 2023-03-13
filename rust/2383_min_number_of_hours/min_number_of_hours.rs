/*
 * @Date: 2023-03-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-13
 * @FilePath: /algorithm/rust/2383_min_number_of_hours/min_number_of_hours.rs
 */

pub fn min_number_of_hours(
    initial_energy: i32,
    mut initial_experience: i32,
    energy: Vec<i32>,
    experience: Vec<i32>,
) -> i32 {
    let (mut min_hours, total_energy) = (0, energy.iter().sum::<i32>());
    min_hours += if total_energy - initial_energy >= 0 {
        total_energy - initial_energy + 1
    } else {
        0
    };
    for e in experience {
        if initial_experience <= e {
            min_hours += e - initial_experience + 1;
            initial_experience = e + 1;
        }
        initial_experience += e;
    }
    min_hours
}

fn main() {
    {
        let initial_energy = 5;
        let initial_experience = 3;
        let energy = vec![1, 4, 3, 2];
        let experience = vec![2, 6, 3, 1];
        let ans = 8;
        assert_eq!(
            min_number_of_hours(initial_energy, initial_experience, energy, experience),
            ans
        );
    }

    {
        let initial_energy = 2;
        let initial_experience = 4;
        let energy = vec![1];
        let experience = vec![3];
        let ans = 0;
        assert_eq!(
            min_number_of_hours(initial_energy, initial_experience, energy, experience),
            ans
        );
    }
}

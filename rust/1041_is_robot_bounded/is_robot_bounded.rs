/*
 * @Date: 2023-04-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-11
 * @FilePath: /algorithm/rust/1041_is_robot_bounded/is_robot_bounded.rs
 */

pub fn is_robot_bounded(instructions: String) -> bool {
    let directions = [[0, 1], [-1, 0], [0, -1], [1, 0]];
    let (x, y, i) = instructions.bytes().fold((0, 0, 0), |(x, y, i), ch| {
        if ch == b'L' {
            (x, y, (i + 3) % 4)
        } else if ch == b'R' {
            (x, y, (i + 1) % 4)
        } else {
            (x + directions[i][0], y + directions[i][1], i)
        }
    });
    x == 0 && y == 0 || i > 0
}

fn main() {
    {
        let instructions = String::from("GGLLGG");
        assert_eq!(is_robot_bounded(instructions), true);
    }

    {
        let instructions = String::from("GG");
        assert_eq!(is_robot_bounded(instructions), false);
    }

    {
        let instructions = String::from("GL");
        assert_eq!(is_robot_bounded(instructions), true);
    }
}

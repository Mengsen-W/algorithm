use std::collections::HashMap;

struct Robot {
    moved: bool,
    idx: usize,
    pos: Vec<(i32, i32)>,
    dir: Vec<i32>,
    to_dir: HashMap<i32, String>,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let mut pos = Vec::new();
        let mut dir = Vec::new();
        let mut to_dir = HashMap::new();

        to_dir.insert(0, "East".to_string());
        to_dir.insert(1, "North".to_string());
        to_dir.insert(2, "West".to_string());
        to_dir.insert(3, "South".to_string());

        for i in 0..width {
            pos.push((i, 0));
            dir.push(0);
        }
        for i in 1..height {
            pos.push((width - 1, i));
            dir.push(1);
        }
        for i in (0..=width-2).rev() {
            pos.push((i, height - 1));
            dir.push(2);
        }
        for i in (1..=height-2).rev() {
            pos.push((0, i));
            dir.push(3);
        }
        dir[0] = 3;

        Robot {
            moved: false,
            idx: 0,
            pos,
            dir,
            to_dir,
        }
    }

    fn step(&mut self, num: i32) {
        self.moved = true;
        self.idx = (self.idx + num as usize) % self.pos.len();
    }

    fn get_pos(&self) -> Vec<i32> {
        vec![self.pos[self.idx].0, self.pos[self.idx].1]
    }

    fn get_dir(&self) -> String {
        if !self.moved {
            return "East".to_string();
        }
        self.to_dir.get(&self.dir[self.idx]).unwrap().clone()
    }
}

fn main() {
    let mut robot = Robot::new(6, 3); // 初始化网格图，机器人在 (0, 0) ，朝东。
    robot.step(2);  // 机器人朝东移动 2 步，到达 (2, 0) ，并朝东。
    robot.step(2);  // 机器人朝东移动 2 步，到达 (4, 0) ，并朝东。
    robot.get_pos(); // 返回 [4, 0]
    robot.get_dir(); // 返回 "East"
    robot.step(2);  // 朝东移动 1 步到达 (5, 0) ，并朝东。
    // 下一步继续往东移动将出界，所以逆时针转变方向朝北。
    // 然后，往北移动 1 步到达 (5, 1) ，并朝北。
    robot.step(1);  // 朝北移动 1 步到达 (5, 2) ，并朝 北 （不是朝西）。
    robot.step(4);  // 下一步继续往北移动将出界，所以逆时针转变方向朝西。
    // 然后，移动 4 步到 (1, 2) ，并朝西。
    robot.get_pos(); // 返回 [1, 2]
    robot.get_dir(); // 返回 "West"
}

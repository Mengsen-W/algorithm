// Package main ...
package main

type Robot struct {
    moved bool
    idx   int
    pos   [][2]int
    dir   []int
    toDir map[int]string
}

func Constructor(width int, height int) Robot {
    robot := Robot{
        toDir: map[int]string{
            0: "East",
            1: "North",
            2: "West",
            3: "South",
        },
    }
    
    for i := 0; i < width; i++ {
        robot.pos = append(robot.pos, [2]int{i, 0})
        robot.dir = append(robot.dir, 0)
    }
    for i := 1; i < height; i++ {
        robot.pos = append(robot.pos, [2]int{width - 1, i})
        robot.dir = append(robot.dir, 1)
    }
    for i := width - 2; i >= 0; i-- {
        robot.pos = append(robot.pos, [2]int{i, height - 1})
        robot.dir = append(robot.dir, 2)
    }
    for i := height - 2; i > 0; i-- {
        robot.pos = append(robot.pos, [2]int{0, i})
        robot.dir = append(robot.dir, 3)
    }
    robot.dir[0] = 3
    
    return robot
}

func (this *Robot) Step(num int) {
    this.moved = true
    this.idx = (this.idx + num) % len(this.pos)
}

func (this *Robot) GetPos() []int {
    return []int{this.pos[this.idx][0], this.pos[this.idx][1]}
}

func (this *Robot) GetDir() string {
    if !this.moved {
        return "East"
    }
    return this.toDir[this.dir[this.idx]]
}

func main() {
	robot := Constructor(6, 3);  // 初始化网格图，机器人在 (0, 0) ，朝东。
  robot.Step(2);              // 机器人朝东移动 2 步，到达 (2, 0) ，并朝东。
  robot.Step(2);              // 机器人朝东移动 2 步，到达 (4, 0) ，并朝东。
  robot.GetPos();             // 返回 [4, 0]
  robot.GetDir();             // 返回 "East"
  robot.Step(2);              // 朝东移动 1 步到达 (5, 0) ，并朝东。
                              // 下一步继续往东移动将出界，所以逆时针转变方向朝北。
                              // 然后，往北移动 1 步到达 (5, 1) ，并朝北。
  robot.Step(1);              // 朝北移动 1 步到达 (5, 2) ，并朝 北 （不是朝西）。
  robot.Step(4);              // 下一步继续往北移动将出界，所以逆时针转变方向朝西。
                              // 然后，移动 4 步到 (1, 2) ，并朝西。
  robot.GetPos();             // 返回 [1, 2]
  robot.GetDir();             // 返回 "West"
}

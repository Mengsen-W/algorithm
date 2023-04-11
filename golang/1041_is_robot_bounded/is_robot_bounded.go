/*
 * @Date: 2023-04-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-11
 * @FilePath: /algorithm/golang/1041_is_robot_bounded/is_robot_bounded.go
 */

// Package main ...
package main

func isRobotBounded(instructions string) bool {
	direc := [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	direcIndex := 0
	x, y := 0, 0
	n := len(instructions)
	for i := 0; i < n; i++ {
		instruction := instructions[i]
		if instruction == 'G' {
			x += direc[direcIndex][0]
			y += direc[direcIndex][1]
		} else if instruction == 'L' {
			direcIndex += 3
			direcIndex %= 4
		} else {
			direcIndex++
			direcIndex %= 4
		}
	}
	return direcIndex != 0 || (x == 0 && y == 0)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		instructions := "GGLLGG"
		assert(isRobotBounded(instructions))
	}

	{
		instructions := "GG"
		assert(!isRobotBounded(instructions))
	}

	{
		instructions := "GL"
		assert(isRobotBounded(instructions))
	}
}

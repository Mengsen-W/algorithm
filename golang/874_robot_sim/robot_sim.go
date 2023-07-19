/*
 * @Date: 2023-07-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-19
 * @FilePath: /algorithm/golang/874_robot_sim/robot_sim.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func robotSim(commands []int, obstacles [][]int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	dirs := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	px, py, d := 0, 0, 1
	set := make(map[int]bool)
	for _, obstacle := range obstacles {
		set[obstacle[0]*60001+obstacle[1]] = true
	}
	res := 0
	for _, c := range commands {
		if c < 0 {
			if c == -1 {
				d = (d + 1) % 4
			} else {
				d = (d + 3) % 4
			}
		} else {
			for i := 0; i < c; i++ {
				if set[(px+dirs[d][0])*60001+py+dirs[d][1]] {
					break
				}
				px += dirs[d][0]
				py += dirs[d][1]
				res = max(res, px*px+py*py)
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		commands  []int
		obstacles [][]int
		ans       int
	}{
		{[]int{4, -1, 3}, [][]int{}, 25},
		{[]int{4, -1, 4, -2, 4}, [][]int{{2, 4}}, 65},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, robotSim(item.commands, item.obstacles), item.ans)
	}
}

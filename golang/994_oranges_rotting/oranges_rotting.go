/*
 * @Date: 2024-05-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-13
 * @FilePath: /algorithm/golang/994_oranges_rotting/oranges_rotting.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func orangesRotting(grid [][]int) int {
	R, C := len(grid), len(grid[0])
	dr := []int{-1, 0, 1, 0}
	dc := []int{0, -1, 0, 1}
	queue := []int{}
	depth := make(map[int]int)

	for r := 0; r < R; r++ {
		for c := 0; c < C; c++ {
			if grid[r][c] == 2 {
				code := r*C + c
				queue = append(queue, code)
				depth[code] = 0
			}
		}
	}

	ans := 0
	for len(queue) > 0 {
		code := queue[0]
		queue = queue[1:]
		r, c := code/C, code%C
		for k := 0; k < 4; k++ {
			nr, nc := r+dr[k], c+dc[k]
			if nr >= 0 && nr < R && nc >= 0 && nc < C && grid[nr][nc] == 1 {
				grid[nr][nc] = 2
				ncode := nr*C + nc
				queue = append(queue, ncode)
				depth[ncode] = depth[code] + 1
				ans = depth[ncode]
			}
		}
	}

	for _, row := range grid {
		for _, v := range row {
			if v == 1 {
				return -1
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{2, 1, 1}, {1, 1, 0}, {0, 1, 1}}, 4},
		{[][]int{{2, 1, 1}, {0, 1, 1}, {1, 0, 1}}, -1},
		{[][]int{{0, 2}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, orangesRotting(test.grid), index)
	}
}

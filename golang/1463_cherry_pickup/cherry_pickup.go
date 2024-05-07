/*
 * @Date: 2024-05-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-07
 * @FilePath: /algorithm/golang/1463_cherry_pickup/cherry_pickup.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func cherryPickup(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	f := make([][]int, n)
	g := make([][]int, n)
	for i := range f {
		f[i] = make([]int, n)
		g[i] = make([]int, n)
		for j := range f[i] {
			f[i][j] = -1
			g[i][j] = -1
		}
	}

	f[0][n-1] = grid[0][0] + grid[0][n-1]
	for i := 1; i < m; i++ {
		for j1 := 0; j1 < n; j1++ {
			for j2 := 0; j2 < n; j2++ {
				best := -1
				for dj1 := j1 - 1; dj1 <= j1+1; dj1++ {
					for dj2 := j2 - 1; dj2 <= j2+1; dj2++ {
						if dj1 >= 0 && dj1 < n && dj2 >= 0 && dj2 < n && f[dj1][dj2] != -1 {
							if j1 == j2 {
								best = max(best, f[dj1][dj2]+grid[i][j1])
							} else {
								best = max(best, f[dj1][dj2]+grid[i][j1]+grid[i][j2])
							}
						}
					}
				}
				g[j1][j2] = best
			}
		}
		f, g = g, f
	}

	ans := 0
	for _, row := range f {
		for _, v := range row {
			ans = max(ans, v)
		}
	}
	return ans
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{3, 1, 1}, {2, 5, 1}, {1, 5, 5}, {2, 1, 1}}, 24},
		{
			[][]int{
				{1, 0, 0, 0, 0, 0, 1},
				{2, 0, 0, 0, 0, 3, 0},
				{2, 0, 9, 0, 0, 0, 0},
				{0, 3, 0, 5, 4, 0, 0},
				{1, 0, 2, 3, 0, 0, 6},
			},
			28,
		},
		{[][]int{{1, 0, 0, 3}, {0, 0, 0, 3}, {0, 0, 3, 3}, {9, 0, 3, 3}}, 22},
		{[][]int{{1, 1}, {1, 1}}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, cherryPickup(test.grid), index)
	}
}

/*
 * @Date: 2023-09-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-13
 * @FilePath: /algorithm/golang/2596_check_valid_grid/check_valid_grid.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func checkValidGrid(grid [][]int) bool {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	if grid[0][0] != 0 {
		return false
	}
	n := len(grid)
	indices := make([][2]int, n*n)
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			indices[grid[i][j]][0] = i
			indices[grid[i][j]][1] = j
		}
	}
	for i := 1; i < n*n; i++ {
		dx := abs(indices[i][0] - indices[i-1][0])
		dy := abs(indices[i][1] - indices[i-1][1])
		if dx*dy != 2 {
			return false
		}
	}
	return true
}

func main() {
	tests := []struct {
		grid [][]int
		ans  bool
	}{
		{[][]int{{0, 11, 16, 5, 20}, {17, 4, 19, 10, 15}, {12, 1, 8, 21, 6}, {3, 18, 23, 14, 9}, {24, 13, 2, 7, 22}}, true},
		{[][]int{{0, 3, 6}, {5, 8, 1}, {2, 7, 4}}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, checkValidGrid(test.grid), index)
	}
}

/*
 * @Date: 2024-04-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-27
 * @FilePath: /algorithm/golang/2639_find_column_width/find_column_width.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findColumnWidth(grid [][]int) []int {
	n, m := len(grid), len(grid[0])
	res := make([]int, m)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			x, length := grid[i][j], 0
			if x <= 0 {
				length = 1
			}
			for x != 0 {
				length, x = length+1, x/10
			}
			res[j] = max(res[j], length)
		}
	}
	return res
}

func main() {
	tests := []struct {
		grid [][]int
		ans  []int
	}{
		{[][]int{{1}, {22}, {333}}, []int{3}},
		{[][]int{{-15, 1, 3}, {15, 7, 12}, {5, 6, -2}}, []int{3, 1, 2}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findColumnWidth(test.grid), index)
	}
}

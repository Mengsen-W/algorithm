/*
 * @Date: 2023-07-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-27
 * @FilePath: /algorithm/golang/2500_delete_greatest_value/delete_greatest_value.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func deleteGreatestValue(grid [][]int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	m := len(grid)
	n := len(grid[0])
	for i := 0; i < m; i++ {
		sort.Ints(grid[i])
	}
	res := 0
	for j := 0; j < n; j++ {
		mx := 0
		for i := 0; i < m; i++ {
			mx = max(mx, grid[i][j])
		}
		res += mx
	}
	return res
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{1, 2, 4}, {3, 3, 1}}, 8},
		{[][]int{{10}}, 10},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, deleteGreatestValue(item.grid))
	}
}

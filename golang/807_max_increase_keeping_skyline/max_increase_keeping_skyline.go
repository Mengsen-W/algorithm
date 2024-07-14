/*
 * @Date: 2021-12-13 01:39:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-13 01:50:39
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxIncreaseKeepingSkyline(grid [][]int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	n := len(grid)
	rowMax := make([]int, n)
	colMax := make([]int, n)
	for i, row := range grid {
		for j, h := range row {
			rowMax[i] = max(rowMax[i], h)
			colMax[j] = max(colMax[j], h)
		}
	}
	for i, row := range grid {
		for j, h := range row {
			ans += min(rowMax[i], colMax[j]) - h
		}
	}
	return
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{3, 0, 8, 4}, {2, 4, 5, 7}, {9, 2, 6, 3}, {0, 3, 1, 0}}, 35},
		{[][]int{{0, 0, 0}, {0, 0, 0}, {0, 0, 0}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxIncreaseKeepingSkyline(test.grid), index)
	}
}

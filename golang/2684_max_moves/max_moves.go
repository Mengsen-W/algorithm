/*
 * @Date: 2024-03-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-16
 * @FilePath: /algorithm/golang/2684_max_moves/max_moves.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxMoves(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	for _, row := range grid {
		row[0] *= -1 // 入队标记
	}
	for j := 0; j < n-1; j++ {
		ok := false
		for i := 0; i < m; i++ {
			if grid[i][j] > 0 { // 不在队列中
				continue
			}
			for k := max(i-1, 0); k < min(i+2, m); k++ {
				if grid[k][j+1] > -grid[i][j] {
					grid[k][j+1] *= -1 // 入队标记
					ok = true
				}
			}
		}
		if !ok { // 无法再往右走了
			return j
		}
	}
	return n - 1
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{2, 4, 3, 5}, {5, 4, 9, 3}, {3, 4, 2, 11}, {10, 9, 13, 15}}, 3},
		{[][]int{{3, 2, 4}, {2, 1, 9}, {1, 1, 7}}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxMoves(test.grid), index)
	}
}

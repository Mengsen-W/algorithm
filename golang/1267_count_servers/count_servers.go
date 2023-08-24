/*
 * @Date: 2023-08-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-24
 * @FilePath: /algorithm/golang/1267_count_servers/count_servers.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countServers(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	rows, cols := make(map[int]int, m), make(map[int]int, n)
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == 1 {
				rows[i] = rows[i] + 1
				cols[j] = cols[j] + 1
			}
		}
	}
	ans := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == 1 && (rows[i] > 1 || cols[j] > 1) {
				ans++
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
		{[][]int{{1, 0}, {0, 1}}, 0},
		{[][]int{{1, 0}, {1, 1}}, 3},
		{[][]int{{1, 1, 0, 0}, {0, 0, 1, 0}, {0, 0, 1, 0}, {0, 0, 0, 1}}, 4},
	}

	for index, item := range tests {
		assert.Equal(&testing.T{}, item.ans, countServers(item.grid), index)
	}
}

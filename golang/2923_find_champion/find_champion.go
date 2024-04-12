/*
 * @Date: 2024-04-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-12
 * @FilePath: /algorithm/golang/2923_find_champion/find_champion.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findChampion(grid [][]int) int {
	n := len(grid)
	for i := 0; i < n; i++ {
		sum := 0
		for _, val := range grid[i] {
			sum += val
		}
		if sum == n-1 {
			return i
		}
	}
	return -1
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{0, 1}, {0, 0}}, 0},
		{[][]int{{0, 0, 1}, {1, 0, 1}, {0, 0, 0}}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findChampion(test.grid), index)
	}
}

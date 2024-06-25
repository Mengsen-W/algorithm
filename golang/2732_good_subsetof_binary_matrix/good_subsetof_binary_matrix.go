// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func goodSubsetofBinaryMatrix(grid [][]int) []int {
	ans := []int{}
	mp := make(map[int]int)
	m := len(grid)
	n := len(grid[0])
	for j := 0; j < m; j++ {
		st := 0
		for i := 0; i < n; i++ {
			st |= (grid[j][i] << i)
		}
		mp[st] = j
	}
	if _, ok := mp[0]; ok {
		ans = append(ans, mp[0])
		return ans
	}
	for x, i := range mp {
		for y, j := range mp {
			if x&y == 0 {
				return []int{min(i, j), max(i, j)}
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		grid [][]int
		ans  []int
	}{
		{[][]int{{0, 1, 1, 0}, {0, 0, 0, 1}, {1, 1, 1, 1}}, []int{0, 1}},
		{[][]int{{0}}, []int{0}},
		{[][]int{{1, 1, 1}, {1, 1, 1}}, []int{}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, goodSubsetofBinaryMatrix(test.grid), index)
	}
}

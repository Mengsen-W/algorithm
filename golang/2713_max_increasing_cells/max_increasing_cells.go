// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxIncreasingCells(mat [][]int) int {
	m, n := len(mat), len(mat[0])
	mp := make(map[int][][2]int)
	row := make([]int, m)
	col := make([]int, n)
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			mp[mat[i][j]] = append(mp[mat[i][j]], [2]int{i, j})
		}
	}
	keys := []int{}
	for key := range mp {
		keys = append(keys, key)
	}
	sort.Ints(keys)

	for _, key := range keys {
		pos := mp[key]
		res := []int{} // 存放相同数值的答案，便于后续更新 row 和 col
		for _, arr := range pos {
			res = append(res, max(row[arr[0]], col[arr[1]])+1)
		}
		for i := 0; i < len(pos); i++ {
			arr, d := pos[i], res[i]
			row[arr[0]] = max(row[arr[0]], d)
			col[arr[1]] = max(col[arr[1]], d)
		}
	}

	return func(slice []int) int {
		maxVal := slice[0]
		for _, val := range slice {
			if val > maxVal {
				maxVal = val
			}
		}
		return maxVal
	}(row)
}

func main() {
	tests := []struct {
		mat [][]int
		ans int
	}{
		{[][]int{{3, 1}, {3, 4}}, 2},
		{[][]int{{1, 1}, {1, 1}}, 1},
		{[][]int{{3, 1, 6}, {-9, 5, 7}}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxIncreasingCells(test.mat), index)
	}
}

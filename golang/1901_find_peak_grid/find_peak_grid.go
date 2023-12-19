/*
 * @Date: 2023-12-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-19
 * @FilePath: /algorithm/golang/1901_find_peak_grid/find_peak_grid.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findPeakGrid(mat [][]int) []int {
	maxElement := func(row []int) int {
		i := 0
		for j := range row {
			if row[i] < row[j] {
				i = j
			}
		}
		return i
	}
	m := len(mat)
	low, high := 0, m-1
	for low <= high {
		i := (low + high) / 2
		j := maxElement(mat[i])
		if i-1 >= 0 && mat[i][j] < mat[i-1][j] {
			high = i - 1
			continue
		}
		if i+1 < m && mat[i][j] < mat[i+1][j] {
			low = i + 1
			continue
		}
		return []int{i, j}
	}
	return nil // impossible
}

func main() {
	tests := []struct {
		mat [][]int
		ans []int
	}{
		{[][]int{{1, 4}, {3, 2}}, []int{0, 1}},
		{[][]int{{10, 20, 15}, {21, 30, 14}, {7, 16, 32}}, []int{1, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findPeakGrid(test.mat), index)
	}
}

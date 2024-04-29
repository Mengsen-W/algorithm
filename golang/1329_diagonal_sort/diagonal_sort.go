/*
 * @Date: 2024-04-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-29
 * @FilePath: /algorithm/golang/1329_diagonal_sort/diagonal_sort.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func diagonalSort(mat [][]int) [][]int {
	n := len(mat)
	m := len(mat[0])
	diag := make([][]int, m+n)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			diag[i-j+m] = append(diag[i-j+m], mat[i][j])
		}
	}
	for _, d := range diag {
		sort.Sort(sort.Reverse(sort.IntSlice(d)))
	}
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			mat[i][j] = diag[i-j+m][len(diag[i-j+m])-1]
			diag[i-j+m] = diag[i-j+m][:len(diag[i-j+m])-1]
		}
	}
	return mat
}

func main() {
	tests := []struct {
		mat [][]int
		ans [][]int
	}{
		{[][]int{{3, 3, 1, 1}, {2, 2, 1, 2}, {1, 1, 1, 2}}, [][]int{{1, 1, 1, 1}, {1, 2, 2, 2}, {1, 2, 3, 3}}},
		{
			[][]int{{11, 25, 66, 1, 69, 7}, {23, 55, 17, 45, 15, 52}, {75, 31, 36, 44, 58, 8}, {22, 27, 33, 25, 68, 4}, {84, 28, 14, 11, 5, 50}},
			[][]int{{5, 17, 4, 1, 52, 7}, {11, 11, 25, 45, 8, 69}, {14, 23, 25, 44, 58, 15}, {22, 27, 31, 36, 50, 66}, {84, 28, 75, 33, 55, 68}},
		},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, diagonalSort(test.mat), index)
	}
}

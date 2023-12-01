/*
 * @Date: 2023-12-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-01
 * @FilePath: /algorithm/golang/2661_first_complete_index/first_complete_index.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func firstCompleteIndex(arr []int, mat [][]int) int {
	n, m := len(mat), len(mat[0])
	mp := make(map[int][2]int)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			mp[mat[i][j]] = [2]int{i, j}
		}
	}
	rowCnt, colCnt := make([]int, n), make([]int, m)
	for i := 0; i < n; i++ {
		rowCnt[i] = 0
	}
	for j := 0; j < m; j++ {
		colCnt[j] = 0
	}
	for i := 0; i < len(arr); i++ {
		v := mp[arr[i]]
		rowCnt[v[0]]++
		if rowCnt[v[0]] == m {
			return i
		}
		colCnt[v[1]]++
		if colCnt[v[1]] == n {
			return i
		}
	}
	return -1
}

func main() {
	tests := []struct {
		arr []int
		mat [][]int
		ans int
	}{
		{[]int{1, 3, 4, 2}, [][]int{{1, 4}, {2, 3}}, 2},
		{[]int{2, 8, 7, 4, 1, 3, 5, 6, 9}, [][]int{{3, 2, 5}, {1, 4, 6}, {8, 7, 9}}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, firstCompleteIndex(test.arr, test.mat), index)
	}
}

/*
 * @Date: 2023-03-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-14
 * @FilePath: /algorithm/golang/1605_restore_matrix/restore_matrix.go
 */

// Package main ...
package main

import (
	"reflect"
)

func restoreMatrix(rowSum []int, colSum []int) [][]int {
	n, m := len(rowSum), len(colSum)
	matrix := make([][]int, n)
	for i := range matrix {
		matrix[i] = make([]int, m)
	}
	i, j := 0, 0
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	for i < n && j < m {
		v := min(rowSum[i], colSum[j])
		matrix[i][j] = v
		rowSum[i] -= v
		colSum[j] -= v
		if rowSum[i] == 0 {
			i++
		}
		if colSum[j] == 0 {
			j++
		}
	}
	return matrix
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		rowSum := []int{3, 8}
		colSum := []int{4, 7}
		ans := [][]int{{3, 0}, {1, 7}}
		assert(restoreMatrix(rowSum, colSum), ans)
	}

	{
		rowSum := []int{5, 7, 10}
		colSum := []int{8, 6, 8}
		ans := [][]int{{5, 0, 0}, {3, 4, 0}, {0, 2, 8}}
		assert(restoreMatrix(rowSum, colSum), ans)
	}

	{
		rowSum := []int{14, 9}
		colSum := []int{6, 9, 8}
		ans := [][]int{{6, 8, 0}, {0, 1, 8}}
		assert(restoreMatrix(rowSum, colSum), ans)
	}

	{
		rowSum := []int{1, 0}
		colSum := []int{1}
		ans := [][]int{{1}, {0}}
		assert(restoreMatrix(rowSum, colSum), ans)
	}

	{
		rowSum := []int{0}
		colSum := []int{0}
		ans := [][]int{{0}}
		assert(restoreMatrix(rowSum, colSum), ans)
	}
}

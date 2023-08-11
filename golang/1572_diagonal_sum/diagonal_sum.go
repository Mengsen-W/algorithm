/*
 * @Date: 2023-08-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-11
 * @FilePath: /algorithm/golang/1572_diagonal_sum/diagonal_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func diagonalSum(mat [][]int) int {
	n := len(mat)
	sum := 0
	mid := n / 2
	for i := 0; i < n; i++ {
		sum += mat[i][i] + mat[i][n-1-i]
	}
	if n&1 == 1 {
		sum -= mat[mid][mid]
	}
	return sum
}

func main() {
	tests := []struct {
		mat [][]int
		ans int
	}{
		{[][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, 25},
		{[][]int{{1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}, {1, 1, 1, 1}}, 8},
		{[][]int{{5}}, 5},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, diagonalSum(item.mat))
	}
}

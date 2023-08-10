/*
 * @Date: 2023-08-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-10
 * @FilePath: /algorithm/golang/1289_min_falling_path_sum/min_falling_path_sum.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minFallingPathSum(grid [][]int) int {
	n := len(grid)
	firstMinSum, secondMinSum := 0, 0
	firstMinIndex := -1

	for i := 0; i < n; i++ {
		curFirstMinSum, curSecondMinSum := math.MaxInt, math.MaxInt
		curFirstMinIndex := -1

		for j := 0; j < n; j++ {
			curSum := grid[i][j]
			if j != firstMinIndex {
				curSum += firstMinSum
			} else {
				curSum += secondMinSum
			}
			if curSum < curFirstMinSum {
				curSecondMinSum, curFirstMinSum = curFirstMinSum, curSum
				curFirstMinIndex = j
			} else if curSum < curSecondMinSum {
				curSecondMinSum = curSum
			}
		}
		firstMinSum, secondMinSum = curFirstMinSum, curSecondMinSum
		firstMinIndex = curFirstMinIndex
	}
	return firstMinSum
}

func main() {
	tests := []struct {
		grid [][]int
		ans  int
	}{
		{[][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, 13},
		{[][]int{{7}}, 7},
	}
	for _, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minFallingPathSum(test.grid))
	}
}

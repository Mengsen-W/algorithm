/*
 * @Date: 2023-07-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-04
 * @FilePath: /algorithm/golang/2679_matrix_sum/matrix_sum.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func matrixSum(nums [][]int) int {
	res := 0
	m := len(nums)
	n := len(nums[0])
	for i := 0; i < m; i++ {
		sort.Ints(nums[i])
	}
	for j := 0; j < n; j++ {
		maxVal := 0
		for i := 0; i < m; i++ {
			if nums[i][j] > maxVal {
				maxVal = nums[i][j]
			}
		}
		res += maxVal
	}
	return res
}

func main() {
	testMap := []struct {
		nums [][]int
		ans  int
	}{
		{[][]int{{7, 2, 1}, {6, 4, 2}, {6, 5, 3}, {3, 2, 1}}, 15},
		{[][]int{{1}}, 1},
	}

	for _, item := range testMap {
		assert.Equal(&testing.T{}, matrixSum(item.nums), item.ans)
	}
}

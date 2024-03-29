/*
 * @Date: 2024-03-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-29
 * @FilePath: /algorithm/golang/2908_minimum_sum/minimum_sum.go
 */

// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumSum(nums []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	n := len(nums)
	res := 1000
	mn := 1000
	left := make([]int, n)
	left[0] = 0
	for i := 1; i < n; i++ {
		left[i] = int(math.Min(float64(nums[i-1]), float64(mn)))
		mn = left[i]
	}

	right := nums[n-1]
	for i := n - 2; i > 0; i-- {
		if left[i] < nums[i] && nums[i] > right {
			res = min(res, left[i]+nums[i]+right)
		}
		right = min(right, nums[i])
	}

	if res < 1000 {
		return res
	}
	return -1
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{8, 6, 1, 5, 3}, 9},
		{[]int{5, 4, 8, 7, 10, 2}, 13},
		{[]int{6, 5, 4, 3, 4, 5}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumSum(test.nums), index)
	}
}

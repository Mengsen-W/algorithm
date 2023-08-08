/*
 * @Date: 2023-08-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-08
 * @FilePath: /algorithm/golang/1749_max_absolute_sum/max_absolute_sum.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxAbsoluteSum(nums []int) int {
	max := func(a int, b int) int {
		if a > b {
			return a
		}
		return b
	}
	min := func(a int, b int) int {
		if a < b {
			return a
		}
		return b
	}
	positiveMax, negativeMin := 0, 0
	positiveSum, negativeSum := 0, 0
	for _, num := range nums {
		positiveSum += num
		positiveMax = max(positiveMax, positiveSum)
		positiveSum = max(0, positiveSum)
		negativeSum += num
		negativeMin = min(negativeMin, negativeSum)
		negativeSum = min(0, negativeSum)
	}
	return max(positiveMax, -negativeMin)
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, -3, 2, 3, -4}, 5},
		{[]int{2, -5, 1, -4, 3, -2}, 8},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, maxAbsoluteSum(item.nums))
	}
}

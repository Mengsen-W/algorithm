/*
 * @Date: 2024-04-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-09
 * @FilePath: /algorithm/golang/2529_maximum_count/maximum_count.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumCount(nums []int) int {
	pos1 := lowerBound(nums, 0)
	pos2 := lowerBound(nums, 1)
	return max(pos1, len(nums)-pos2)
}

func lowerBound(nums []int, val int) int {
	l, r := 0, len(nums)
	for l < r {
		m := (l + r) / 2
		if nums[m] >= val {
			r = m
		} else {
			l = m + 1
		}
	}
	return l
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{-2, -1, -1, 1, 2, 3}, 3},
		{[]int{-3, -2, -1, 0, 0, 1, 2}, 3},
		{[]int{5, 20, 66, 1314}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumCount(test.nums), index)
	}
}

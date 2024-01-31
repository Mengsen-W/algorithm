/*
 * @Date: 2024-01-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-31
 * @FilePath: /algorithm/golang/2670_distinct_difference_array/distinct_difference_array.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func distinctDifferenceArray(nums []int) []int {
	st := map[int]struct{}{}
	sufCnt := make([]int, len(nums)+1)
	for i := len(nums) - 1; i > 0; i-- {
		st[nums[i]] = struct{}{}
		sufCnt[i] = len(st)
	}
	var res []int
	st = map[int]struct{}{}
	for i := 0; i < len(nums); i++ {
		st[nums[i]] = struct{}{}
		res = append(res, len(st)-sufCnt[i+1])
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{1, 2, 3, 4, 5}, []int{-3, -1, 1, 3, 5}},
		{[]int{3, 2, 3, 4, 2}, []int{-2, -1, 0, 2, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, distinctDifferenceArray(test.nums), index)
	}
}

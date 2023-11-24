/*
 * @Date: 2023-11-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-24
 * @FilePath: /algorithm/golang/2824_count_pairs/count_pairs.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPairs(nums []int, target int) int {
	sort.Ints(nums)
	res := 0
	for i, j := 0, len(nums)-1; i < j; i++ {
		for i < j && nums[i]+nums[j] >= target {
			j--
		}
		res += j - i
	}
	return res
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    int
	}{
		{[]int{-1, 1, 2, 3, 1}, 2, 3},
		{[]int{-6, 2, 5, -2, -7, -1, 3}, -2, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPairs(test.nums, test.target), index)
	}
}

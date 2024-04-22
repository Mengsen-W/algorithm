/*
 * @Date: 2024-04-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-22
 * @FilePath: /algorithm/golang/377_combination_sum4/combination_sum4.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func combinationSum4(nums []int, target int) int {
	dp := make([]int, target+1)
	dp[0] = 1
	for i := 1; i <= target; i++ {
		for _, num := range nums {
			if num <= i {
				dp[i] += dp[i-num]
			}
		}
	}
	return dp[target]
}

func main() {
	tests := []struct {
		nums   []int
		target int
		ans    int
	}{
		{[]int{1, 2, 3}, 4, 7},
		{[]int{9}, 3, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, combinationSum4(test.nums, test.target), index)
	}
}

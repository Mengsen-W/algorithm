/*
 * @Date: 2024-03-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-01
 * @FilePath: /algorithm/golang/2369_valid_partition/valid_partition.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func validPartition(nums []int) bool {
	n := len(nums)
	dp := make([]bool, n+1)
	dp[0] = true
	for i := 1; i <= n; i++ {
		if i >= 2 {
			dp[i] = dp[i-2] && validTwo(nums[i-2], nums[i-1])
		}
		if i >= 3 {
			dp[i] = dp[i] || (dp[i-3] && validThree(nums[i-3], nums[i-2], nums[i-1]))
		}
	}
	return dp[n]
}

func validTwo(num1, num2 int) bool {
	return num1 == num2
}

func validThree(num1, num2, num3 int) bool {
	return (num1 == num2 && num1 == num3) || (num1+1 == num2 && num2+1 == num3)
}

func main() {
	tests := []struct {
		nums []int
		ans  bool
	}{
		{[]int{4, 4, 4, 5, 6}, true},
		{[]int{1, 1, 1, 2}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, validPartition(test.nums), index)
	}
}

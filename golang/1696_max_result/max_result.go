/*
 * @Date: 2024-02-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-05
 * @FilePath: /algorithm/golang/1696_max_result/max_result.go
 */

package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxResult(nums []int, k int) int {
	n := len(nums)
	dp := make([]int, n)
	dp[0] = nums[0]
	queue := make([]int, n) // 模拟双端队列
	qi, qj := 0, 1
	for i := 1; i < n; i++ {
		for qi < qj && queue[qi] < i-k {
			qi++
		}
		dp[i] = dp[queue[qi]] + nums[i]
		for qi < qj && dp[queue[qj-1]] <= dp[i] {
			qj--
		}
		queue[qj] = i
		qj++
	}
	return dp[n-1]
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, -1, -2, 4, -7, 3}, 2, 7},
		{[]int{10, -5, -2, 4, 0, 3}, 3, 17},
		{[]int{1, -5, -20, 4, -1, 3, -6, -3}, 2, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxResult(test.nums, test.k), index)
	}
}

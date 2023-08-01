/*
 * @Date: 2023-08-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-01
 * @FilePath: /algorithm/golang/2681_sum_of_power/sum_of_power.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func sumOfPower(nums []int) int {
	n := len(nums)
	sort.Ints(nums)
	dp := 0
	preSum := 0
	res := 0
	mod := int(1e9 + 7)
	for i := 0; i < n; i++ {
		dp = (nums[i] + preSum) % mod
		preSum = (preSum + dp) % mod
		res = (res + (nums[i]*nums[i]%mod*dp)%mod) % mod
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 1, 4}, 141},
		{[]int{1, 1, 1}, 7},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, sumOfPower(item.nums))
	}
}

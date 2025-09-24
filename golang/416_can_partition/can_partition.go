// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func canPartition(nums []int) bool {
	n := len(nums)
	if n < 2 {
		return false
	}

	sum, max := 0, 0
	for _, v := range nums {
		sum += v
		if v > max {
			max = v
		}
	}
	if sum%2 != 0 {
		return false
	}

	target := sum / 2
	if max > target {
		return false
	}

	dp := make([]bool, target+1)
	dp[0] = true
	for i := 0; i < n; i++ {
		v := nums[i]
		for j := target; j >= v; j-- {
			dp[j] = dp[j] || dp[j-v]
		}
	}
	return dp[target]
}

func main() {
	tests := []struct {
		nums []int
		ans  bool
	}{
		{[]int{1, 5, 11, 5}, true},
		{[]int{1, 2, 3, 5}, false},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, canPartition(test.nums), index)
	}
}

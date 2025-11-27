// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSubarraySum(nums []int, k int) int64 {
	n := len(nums)
	prefixSum := int64(0)
	maxSum := int64(math.MinInt64)
	kSum := make([]int64, k)
	for i := 0; i < k; i++ {
		kSum[i] = math.MaxInt64 / 2
	}
	kSum[k-1] = 0
	for i := 0; i < n; i++ {
		prefixSum += int64(nums[i])
		maxSum = max(maxSum, prefixSum-kSum[i%k])
		kSum[i%k] = min(kSum[i%k], prefixSum)
	}
	return maxSum
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int64
	}{
		{[]int{1, 2}, 1, 3},
		{[]int{-1, -2, -3, -4, -5}, 4, -10},
		{[]int{-5, 1, 2, -3, 4}, 2, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSubarraySum(test.nums, test.k), index)
	}
}

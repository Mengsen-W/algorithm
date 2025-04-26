// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countSubarrays(nums []int, minK int, maxK int) int64 {
	var res int64
	border, lastMin, lastMax := -1, -1, -1
	for i := 0; i < len(nums); i++ {
		if nums[i] < minK || nums[i] > maxK {
			lastMax, lastMin = -1, -1
			border = i
		}
		if nums[i] == minK {
			lastMin = i
		}
		if nums[i] == maxK {
			lastMax = i
		}
		if lastMin != -1 && lastMax != -1 {
			res += int64(min(lastMin, lastMax) - border)
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		minK int
		maxK int
		ans  int64
	}{
		{[]int{1, 3, 5, 2, 7, 5}, 1, 5, 2},
		{[]int{1, 1, 1, 1}, 1, 1, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countSubarrays(test.nums, test.minK, test.maxK), index)
	}
}

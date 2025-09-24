// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countSubarrays(nums []int) int {
	n := len(nums)
	ans := 0
	for i := 1; i < n-1; i++ {
		if nums[i] == (nums[i-1]+nums[i+1])*2 {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 1, 4, 1}, 1},
		{[]int{1, 1, 1}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countSubarrays(test.nums), index)
	}
}

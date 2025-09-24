// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maximumUniqueSubarray(nums []int) int {
	n := len(nums)
	psum := make([]int, n+1)
	cnt := make(map[int]int)
	ans, pre := 0, 0
	for i := 0; i < n; i++ {
		psum[i+1] = psum[i] + nums[i]
		if val, exists := cnt[nums[i]]; exists {
			pre = max(pre, val)
		}
		ans = max(ans, psum[i+1]-psum[pre])
		cnt[nums[i]] = i + 1
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{4, 2, 4, 5, 6}, 17},
		{[]int{5, 2, 1, 2, 5, 2, 1, 2, 5}, 8},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maximumUniqueSubarray(test.nums), index)
	}
}

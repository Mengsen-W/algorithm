// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func resultsArray(nums []int, k int) []int {
	n := len(nums)
	cnt := 0
	ans := make([]int, n-k+1)
	for i := 0; i < n; i++ {
		if i == 0 || nums[i]-nums[i-1] != 1 {
			cnt = 1
		} else {
			cnt++
		}
		if cnt >= k {
			ans[i-k+1] = nums[i]
		} else if i-k+1 >= 0 {
			ans[i-k+1] = -1
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  []int
	}{
		{[]int{1, 2, 3, 4, 3, 2, 5}, 3, []int{3, 4, -1, -1, -1}},
		{[]int{2, 2, 2, 2, 2}, 4, []int{-1, -1}},
		{[]int{3, 2, 3, 2, 3, 2}, 2, []int{-1, 3, -1, 3, -1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, resultsArray(test.nums, test.k), index)
	}
}

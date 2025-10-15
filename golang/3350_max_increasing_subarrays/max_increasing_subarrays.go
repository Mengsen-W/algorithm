// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxIncreasingSubarrays(nums []int) int {
	n := len(nums)
	cnt, precnt, ans := 1, 0, 0
	for i := 1; i < n; i++ {
		if nums[i] > nums[i-1] {
			cnt++
		} else {
			precnt = cnt
			cnt = 1
		}
		ans = max(ans, min(precnt, cnt))
		ans = max(ans, cnt/2)
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 5, 7, 8, 9, 2, 3, 4, 3, 1}, 3},
		{[]int{1, 2, 3, 4, 4, 4, 4, 5, 6, 7}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxIncreasingSubarrays(test.nums), "case %d", index)
	}
}

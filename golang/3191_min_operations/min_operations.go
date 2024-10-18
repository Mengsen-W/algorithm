// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int) int {
	n := len(nums)
	ans := 0
	for i := 0; i < n; i++ {
		if nums[i] == 0 {
			if i > n-3 {
				return -1
			}
			nums[i] ^= 1
			nums[i+1] ^= 1
			nums[i+2] ^= 1
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
		{[]int{0, 1, 1, 1, 0, 0}, 3},
		{[]int{0, 1, 1, 1}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.nums), index)
	}
}

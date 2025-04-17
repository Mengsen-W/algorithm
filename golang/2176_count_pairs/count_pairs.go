// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countPairs(nums []int, k int) int {
	n := len(nums)
	res := 0 // 符合要求数对个数
	for i := 0; i < n-1; i++ {
		for j := i + 1; j < n; j++ {
			if (i*j)%k == 0 && nums[i] == nums[j] {
				res++
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{3, 1, 2, 2, 2, 1, 3}, 2, 4},
		{[]int{1, 2, 3, 4}, 1, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPairs(test.nums, test.k), index)
	}
}

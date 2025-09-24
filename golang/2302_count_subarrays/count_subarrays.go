// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func countSubarrays(nums []int, k int64) int64 {
	n := len(nums)
	res, total := int64(0), int64(0)
	for i, j := 0, 0; j < n; j++ {
		total += int64(nums[j])
		for i <= j && total*int64(j-i+1) >= k {
			total -= int64(nums[i])
			i++
		}
		res += int64(j - i + 1)
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		k    int64
		ans  int64
	}{
		{[]int{2, 1, 4, 3, 5}, 10, 6},
		{[]int{1, 1, 1}, 5, 5},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countSubarrays(test.nums, test.k), index)
	}
}

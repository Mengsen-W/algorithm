// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumSubarrayLength(nums []int, k int) int {
	n := len(nums)
	bits := make([]int, 30)
	res := math.MaxInt32

	for left, right := 0, 0; right < n; right++ {
		for i := 0; i < 30; i++ {
			bits[i] += (nums[right] >> i) & 1
		}
		for left <= right && calc(bits) >= k {
			res = min(res, right-left+1)
			for i := 0; i < 30; i++ {
				bits[i] -= (nums[left] >> i) & 1
			}
			left++
		}
	}

	if res == math.MaxInt32 {
		return -1
	}
	return res
}

func calc(bits []int) int {
	ans := 0
	for i := 0; i < len(bits); i++ {
		if bits[i] > 0 {
			ans |= 1 << i
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{1, 2, 3}, 2, 1},
		{[]int{2, 1, 8}, 10, 3},
		{[]int{1, 2}, 0, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumSubarrayLength(test.nums, test.k), index)
	}
}

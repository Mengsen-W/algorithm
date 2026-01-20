// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minBitwiseArray(nums []int) []int {
	for i, x := range nums {
		res := -1
		d := 1
		for (x & d) != 0 {
			res = x - d
			d <<= 1
		}
		nums[i] = res
	}
	return nums
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{2, 3, 5, 7}, []int{-1, 1, 4, 3}},
		{[]int{11, 13, 31}, []int{9, 12, 15}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minBitwiseArray(test.nums), index)
	}
}

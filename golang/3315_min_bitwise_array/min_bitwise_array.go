// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minBitwiseArray(nums []int) []int {
	for i, x := range nums {
		if x == 2 {
			nums[i] = -1
		} else {
			nums[i] ^= (x + 1) &^ x >> 1
		}
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

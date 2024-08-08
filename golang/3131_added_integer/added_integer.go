// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func addedInteger(nums1 []int, nums2 []int) int {
	maxVal1, maxVal2 := 0, 0

	for _, num := range nums1 {
		if num > maxVal1 {
			maxVal1 = num
		}
	}
	for _, num := range nums2 {
		if num > maxVal2 {
			maxVal2 = num
		}
	}
	return maxVal2 - maxVal1
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   int
	}{
		{[]int{2, 6, 4}, []int{9, 7, 5}, 3},
		{[]int{10}, []int{5}, -5},
		{[]int{1, 1, 1, 1}, []int{1, 1, 1, 1}, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, addedInteger(test.nums1, test.nums2), index)
	}
}

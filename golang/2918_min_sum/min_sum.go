// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minSum(nums1 []int, nums2 []int) int64 {
	var sum1, sum2 int64
	var zero1, zero2 int

	for _, num := range nums1 {
		sum1 += int64(num)
		if num == 0 {
			sum1++
			zero1++
		}
	}

	for _, num := range nums2 {
		sum2 += int64(num)
		if num == 0 {
			sum2++
			zero2++
		}
	}

	if (zero1 == 0 && sum2 > sum1) || (zero2 == 0 && sum1 > sum2) {
		return -1
	}

	if sum1 > sum2 {
		return sum1
	}
	return sum2
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   int64
	}{
		{[]int{3, 2, 0, 1, 0}, []int{6, 5, 0}, 12},
		{[]int{2, 0, 2, 0}, []int{1, 4}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minSum(test.nums1, test.nums2), index)
	}
}

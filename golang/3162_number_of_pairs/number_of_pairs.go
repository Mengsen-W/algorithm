// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func numberOfPairs(nums1 []int, nums2 []int, k int) int {
	res := 0
	for _, a := range nums1 {
		for _, b := range nums2 {
			if a%(b*k) == 0 {
				res++
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		k     int
		ans   int
	}{
		{[]int{1, 3, 4}, []int{1, 3, 4}, 1, 5},
		{[]int{1, 2, 4, 12}, []int{2, 4}, 3, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numberOfPairs(test.nums1, test.nums2, test.k), index)
	}
}

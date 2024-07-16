// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func findIntersectionValues(nums1 []int, nums2 []int) []int {
	s1, s2 := map[int]bool{}, map[int]bool{}
	for _, x := range nums1 {
		s1[x] = true
	}
	for _, x := range nums2 {
		s2[x] = true
	}
	res := make([]int, 2)
	for _, x := range nums1 {
		if s2[x] {
			res[0]++
		}
	}
	for _, x := range nums2 {
		if s1[x] {
			res[1]++
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   []int
	}{
		{[]int{2, 3, 2}, []int{1, 2}, []int{2, 1}},
		{[]int{4, 3, 2, 3, 1}, []int{2, 2, 5, 2, 3, 6}, []int{3, 4}},
		{[]int{3, 4, 2, 3}, []int{1, 5}, []int{0, 0}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, findIntersectionValues(test.nums1, test.nums2), index)
	}
}

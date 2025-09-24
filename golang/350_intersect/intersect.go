// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func intersect(nums1 []int, nums2 []int) []int {
	sort.Ints(nums1)
	sort.Ints(nums2)
	length1, length2 := len(nums1), len(nums2)
	index1, index2 := 0, 0

	intersection := []int{}
	for index1 < length1 && index2 < length2 {
		if nums1[index1] < nums2[index2] {
			index1++
		} else if nums1[index1] > nums2[index2] {
			index2++
		} else {
			intersection = append(intersection, nums1[index1])
			index1++
			index2++
		}
	}
	return intersection
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   []int
	}{
		{[]int{1, 2, 2, 1}, []int{2, 2}, []int{2, 2}},
		{[]int{4, 9, 5}, []int{9, 4, 9, 8, 4}, []int{4, 9}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, intersect(test.nums1, test.nums2), index)
	}
}

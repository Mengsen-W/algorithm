// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minimumAddedInteger(nums1 []int, nums2 []int) int {
	sort.Ints(nums1)
	sort.Ints(nums2)
	for i := 2; i >= 0; i-- {
		left, right := i+1, 1
		for left < len(nums1) && right < len(nums2) {
			if nums1[left]-nums2[right] == nums1[i]-nums2[0] {
				right++
			}
			left++
		}
		if right == len(nums2) {
			return nums2[0] - nums1[i]
		}
	}
	return 0
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   int
	}{
		{[]int{4, 20, 16, 12, 8}, []int{14, 18, 10}, -2},
		{[]int{3, 5, 5, 3}, []int{7, 7}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumAddedInteger(test.nums1, test.nums2), index)
	}
}

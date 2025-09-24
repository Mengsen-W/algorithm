// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func kthSmallestProduct(nums1 []int, nums2 []int, k int64) int64 {
	n1, n2 := len(nums1), len(nums2)
	pos1, pos2 := 0, 0
	for pos1 < n1 && nums1[pos1] < 0 {
		pos1++
	}
	for pos2 < n2 && nums2[pos2] < 0 {
		pos2++
	}
	left, right := int64(-1e10), int64(1e10)
	for left <= right {
		mid := (left + right) / 2
		count := int64(0)
		i1, i2 := 0, pos2-1
		for i1 < pos1 && i2 >= 0 {
			if int64(nums1[i1])*int64(nums2[i2]) > mid {
				i1++
			} else {
				count += int64(pos1 - i1)
				i2--
			}
		}
		i1, i2 = pos1, n2-1
		for i1 < n1 && i2 >= pos2 {
			if int64(nums1[i1])*int64(nums2[i2]) > mid {
				i2--
			} else {
				count += int64(i2 - pos2 + 1)
				i1++
			}
		}
		i1, i2 = 0, pos2
		for i1 < pos1 && i2 < n2 {
			if int64(nums1[i1])*int64(nums2[i2]) > mid {
				i2++
			} else {
				count += int64(n2 - i2)
				i1++
			}
		}
		i1, i2 = pos1, 0
		for i1 < n1 && i2 < pos2 {
			if int64(nums1[i1])*int64(nums2[i2]) > mid {
				i1++
			} else {
				count += int64(n1 - i1)
				i2++
			}
		}
		if count < k {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return left
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		k     int64
		ans   int64
	}{
		{[]int{2, 5}, []int{3, 4}, 2, 8},
		{[]int{-4, -2, 0, 3}, []int{2, 4}, 6, 0},
		{[]int{-2, -1, 0, 1, 2}, []int{-3, -1, 2, 4, 5}, 3, -6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, kthSmallestProduct(test.nums1, test.nums2, test.k), index)
	}
}

/*
 * @Date: 2023-08-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-13
 * @FilePath: /algorithm/golang/88_merge/merge.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func merge(nums1 []int, m int, nums2 []int, n int) {
	for p1, p2, tail := m-1, n-1, m+n-1; p1 >= 0 || p2 >= 0; tail-- {
		var cur int
		if p1 == -1 {
			cur = nums2[p2]
			p2--
		} else if p2 == -1 {
			cur = nums1[p1]
			p1--
		} else if nums1[p1] > nums2[p2] {
			cur = nums1[p1]
			p1--
		} else {
			cur = nums2[p2]
			p2--
		}
		nums1[tail] = cur
	}
}

func main() {
	tests := []struct {
		nums1 []int
		m     int
		nums2 []int
		n     int
		ans   []int
	}{
		{[]int{1, 2, 3, 0, 0, 0}, 3, []int{2, 5, 6}, 3, []int{1, 2, 2, 3, 5, 6}},
		{[]int{1}, 1, []int{}, 0, []int{1}},
		{[]int{0}, 0, []int{1}, 1, []int{1}},
	}

	for _, item := range tests {
		merge(item.nums1, item.m, item.nums2, item.n)
		assert.Equal(&testing.T{}, item.ans, item.nums1)
	}
}

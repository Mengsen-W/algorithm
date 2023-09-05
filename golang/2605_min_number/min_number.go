/*
 * @Date: 2023-09-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-05
 * @FilePath: /algorithm/golang/2605_min_number/min_number.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func minNumber(nums1 []int, nums2 []int) int {
	min := func(x int, y int) int {
		if x > y {
			return y
		}
		return x
	}
	same := func() int {
		s := make(map[int]bool)
		x := 10
		for _, num := range nums1 {
			s[num] = true
		}
		for _, num := range nums2 {
			if _, ok := s[num]; ok {
				x = min(x, num)
			}
		}
		if x == 10 {
			return -1
		}
		return x
	}

	if x := same(); x != -1 {
		return x
	}
	x, y := nums1[0], nums2[0]
	for _, num := range nums1 {
		x = min(x, num)
	}
	for _, num := range nums2 {
		y = min(y, num)
	}
	return min(x*10+y, y*10+x)
}

func main() {
	tests := []struct {
		nums1 []int
		nums2 []int
		ans   int
	}{
		{[]int{4, 1, 3}, []int{5, 7}, 15},
		{[]int{3, 5, 2, 6}, []int{3, 1, 7}, 3},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, minNumber(item.nums1, item.nums2))
	}
}

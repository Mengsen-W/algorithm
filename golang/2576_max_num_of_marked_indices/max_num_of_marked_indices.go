// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxNumOfMarkedIndices(nums []int) int {
	sort.Ints(nums)
	n := len(nums)
	l, r := 0, n/2
	check := func(m int) bool {
		for i := 0; i < m; i++ {
			if nums[i]*2 > nums[n-m+i] {
				return false
			}
		}
		return true
	}

	for l < r {
		m := (l + r + 1) >> 1
		if check(m) {
			l = m
		} else {
			r = m - 1
		}
	}

	return l * 2
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{3, 5, 2, 4}, 2},
		{[]int{9, 2, 5, 4}, 4},
		{[]int{7, 6, 8}, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxNumOfMarkedIndices(test.nums), index)
	}
}

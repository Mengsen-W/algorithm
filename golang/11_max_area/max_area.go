// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxArea(height []int) (ans int) {
	left, right := 0, len(height)-1
	for left < right {
		area := (right - left) * min(height[left], height[right])
		ans = max(ans, area)
		if height[left] < height[right] {
			// height[left] 与右边的任意垂线都无法组成一个比 ans 更大的面积
			left++
		} else {
			// height[right] 与左边的任意垂线都无法组成一个比 ans 更大的面积
			right--
		}
	}
	return
}

func main() {
	tests := []struct {
		height []int
		ans    int
	}{
		{[]int{1, 8, 6, 2, 5, 4, 8, 3, 7}, 49},
		{[]int{1, 1}, 1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxArea(test.height), index)
	}
}

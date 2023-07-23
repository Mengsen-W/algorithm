/*
 * @Date: 2023-07-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-23
 * @FilePath: /algorithm/golang/42_trap/trap.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func trap(height []int) (ans int) {
	left, right := 0, len(height)-1
	leftMax, rightMax := 0, 0
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for left < right {
		leftMax = max(leftMax, height[left])
		rightMax = max(rightMax, height[right])
		if height[left] < height[right] {
			ans += leftMax - height[left]
			left++
		} else {
			ans += rightMax - height[right]
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
		{[]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}, 6},
		{[]int{4, 2, 0, 3, 2, 5}, 9},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, trap(item.height), item.ans)
	}
}

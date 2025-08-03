// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxTotalFruits(fruits [][]int, startPos int, k int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	left := 0
	right := 0
	n := len(fruits)
	sum := 0
	ans := 0

	step := func(left int, right int) int {
		if fruits[right][0] <= startPos {
			return startPos - fruits[left][0]
		} else if fruits[left][0] >= startPos {
			return fruits[right][0] - startPos
		} else {
			return min(abs(startPos-fruits[right][0]), abs(startPos-fruits[left][0])) + fruits[right][0] - fruits[left][0]
		}
	}
	// 每次固定住窗口右边界
	for right < n {
		sum += fruits[right][1]
		// 移动左边界
		for left <= right && step(left, right) > k {
			sum -= fruits[left][1]
			left++
		}
		ans = max(ans, sum)
		right++
	}
	return ans
}

func main() {
	tests := []struct {
		fruits   [][]int
		startPos int
		k        int
		ans      int
	}{
		{[][]int{{2, 8}, {6, 3}, {8, 6}}, 5, 4, 9},
		{[][]int{{0, 9}, {4, 1}, {5, 7}, {6, 2}, {7, 4}, {10, 9}}, 5, 4, 14},
		{[][]int{{0, 3}, {6, 4}, {8, 5}}, 3, 2, 0},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxTotalFruits(test.fruits, test.startPos, test.k), index)
	}
}

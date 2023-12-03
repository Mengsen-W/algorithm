/*
 * @Date: 2023-12-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-03
 * @FilePath: /algorithm/golang/1423_max_score/max_score.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxScore(cardPoints []int, k int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	n := len(cardPoints)
	// 滑动窗口大小为 n-k
	windowSize := n - k
	// 选前 n-k 个作为初始值
	sum := 0
	for _, pt := range cardPoints[:windowSize] {
		sum += pt
	}
	minSum := sum
	for i := windowSize; i < n; i++ {
		// 滑动窗口每向右移动一格，增加从右侧进入窗口的元素值，并减少从左侧离开窗口的元素值
		sum += cardPoints[i] - cardPoints[i-windowSize]
		minSum = min(minSum, sum)
	}
	total := 0
	for _, pt := range cardPoints {
		total += pt
	}
	return total - minSum
}

func main() {
	tests := []struct {
		cardPoints []int
		k          int
		ans        int
	}{
		{[]int{1, 2, 3, 4, 5, 6, 1}, 3, 12},
		{[]int{2, 2, 2}, 2, 4},
		{[]int{9, 7, 7, 9, 7, 7, 9}, 7, 55},
		{[]int{1, 1000, 1}, 1, 1},
		{[]int{1, 79, 80, 1, 1, 1, 200, 1}, 3, 202},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxScore(test.cardPoints, test.k), index)
	}
}

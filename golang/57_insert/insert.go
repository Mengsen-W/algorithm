/*
 * @Date: 2023-08-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-28
 * @FilePath: /algorithm/golang/57_insert/insert.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func insert(intervals [][]int, newInterval []int) (ans [][]int) {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	left, right := newInterval[0], newInterval[1]
	merged := false
	for _, interval := range intervals {
		if interval[0] > right {
			// 在插入区间的右侧且无交集
			if !merged {
				ans = append(ans, []int{left, right})
				merged = true
			}
			ans = append(ans, interval)
		} else if interval[1] < left {
			// 在插入区间的左侧且无交集
			ans = append(ans, interval)
		} else {
			// 与插入区间有交集，计算它们的并集
			left = min(left, interval[0])
			right = max(right, interval[1])
		}
	}
	if !merged {
		ans = append(ans, []int{left, right})
	}
	return
}

func main() {
	tests := []struct {
		intervals   [][]int
		newInterval []int
		ans         [][]int
	}{
		{[][]int{{1, 3}, {6, 9}}, []int{2, 5}, [][]int{{1, 5}, {6, 9}}},
		{[][]int{{1, 2}, {3, 5}, {6, 7}, {8, 10}, {12, 16}}, []int{4, 8}, [][]int{{1, 2}, {3, 10}, {12, 16}}},
		{[][]int{}, []int{5, 7}, [][]int{{5, 7}}},
		{[][]int{{1, 5}}, []int{2, 3}, [][]int{{1, 5}}},
		{[][]int{{1, 5}}, []int{2, 7}, [][]int{{1, 7}}},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, insert(item.intervals, item.newInterval))
	}
}

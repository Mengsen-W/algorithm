/*
 * @Date: 2023-08-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-27
 * @FilePath: /algorithm/golang/56_merge/merge.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func merge(intervals [][]int) (ans [][]int) {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
	for _, interval := range intervals {
		left := interval[0]
		right := interval[1]
		if len(ans) == 0 || ans[len(ans)-1][1] < left {
			ans = append(ans, []int{left, right})
		} else {
			ans[len(ans)-1][1] = max(ans[len(ans)-1][1], right)
		}
	}
	return
}

func main() {
	tests := []struct {
		intervals [][]int
		ans       [][]int
	}{
		{[][]int{{1, 3}, {2, 6}, {8, 10}, {15, 18}}, [][]int{{1, 6}, {8, 10}, {15, 18}}},
		{[][]int{{1, 4}, {4, 5}}, [][]int{{1, 5}}},
		{[][]int{{1, 4}, {0, 4}}, [][]int{{0, 4}}},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, item.ans, merge(item.intervals))
	}
}

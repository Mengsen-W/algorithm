/*
 * @Date: 2023-10-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-27
 * @FilePath: /algorithm/golang/1465_max_area/max_area.go
 */

// Package main ...
package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxArea(h int, w int, horizontalCuts []int, verticalCuts []int) int {
	sort.Ints(horizontalCuts)
	sort.Ints(verticalCuts)

	max := func(x int, y int) int {
		if x < y {
			return y
		}
		return x
	}

	calMax := func(arr []int, boardr int) int {
		res, pre := 0, 0
		for _, i := range arr {
			res = max(i-pre, res)
			pre = i
		}
		return max(res, boardr-pre)
	}

	return calMax(horizontalCuts, h) * calMax(verticalCuts, w) % 1000000007
}

func main() {
	tests := []struct {
		h              int
		w              int
		horizontalCuts []int
		verticalCuts   []int
		ans            int
	}{
		{5, 4, []int{1, 2, 4}, []int{1, 3}, 4},
		{5, 4, []int{3, 1}, []int{1}, 6},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxArea(test.h, test.w, test.horizontalCuts, test.verticalCuts), index)
	}
}

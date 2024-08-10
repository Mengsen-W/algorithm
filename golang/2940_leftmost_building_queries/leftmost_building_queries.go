// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var zd []int

func build(l, r, rt int, heights []int) {
	if l == r {
		zd[rt] = heights[l-1]
		return
	}
	mid := (l + r) >> 1
	build(l, mid, rt<<1, heights)
	build(mid+1, r, rt<<1|1, heights)
	zd[rt] = max(zd[rt<<1], zd[rt<<1|1])
}

func query(pos, val, l, r, rt int) int {
	if val >= zd[rt] {
		return 0
	}
	if l == r {
		return l
	}
	mid := (l + r) >> 1
	if pos <= mid {
		res := query(pos, val, l, mid, rt<<1)
		if res != 0 {
			return res
		}
	}
	return query(pos, val, mid+1, r, rt<<1|1)
}

func leftmostBuildingQueries(heights []int, queries [][]int) []int {
	n := len(heights)
	zd = make([]int, n*4)
	build(1, n, 1, heights)

	m := len(queries)
	ans := make([]int, m)
	for i := 0; i < m; i++ {
		a, b := queries[i][0], queries[i][1]
		if a > b {
			a, b = b, a
		}
		if a == b || heights[a] < heights[b] {
			ans[i] = b
			continue
		}
		ans[i] = query(b+1, heights[a], 1, n, 1) - 1
	}
	return ans
}

func main() {
	tests := []struct {
		heights []int
		queries [][]int
		ans     []int
	}{
		{[]int{6, 4, 8, 5, 2, 7}, [][]int{{0, 1}, {0, 3}, {2, 4}, {3, 4}, {2, 2}}, []int{2, 5, -1, 5, 2}},
		{[]int{5, 3, 8, 2, 6, 1, 4, 6}, [][]int{{0, 7}, {3, 5}, {5, 2}, {3, 0}, {1, 6}}, []int{7, 6, -1, 4, 6}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, leftmostBuildingQueries(test.heights, test.queries), index)
	}
}

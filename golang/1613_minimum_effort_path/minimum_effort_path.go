/*
 * @Date: 2023-12-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-11
 * @FilePath: /algorithm/golang/1613_minimum_effort_path/minimum_effort_path.go
 */

// Package main ...
package main

import (
	"container/heap"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

type (
	point struct{ x, y, maxDiff int }
	hp    []point
)

func (h hp) Len() int              { return len(h) }
func (h hp) Less(i, j int) bool    { return h[i].maxDiff < h[j].maxDiff }
func (h hp) Swap(i, j int)         { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{})   { *h = append(*h, v.(point)) }
func (h *hp) Pop() (v interface{}) { a := *h; *h, v = a[:len(a)-1], a[len(a)-1]; return }

type pair struct{ x, y int }

var dir4 = []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

func minimumEffortPath(heights [][]int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	n, m := len(heights), len(heights[0])
	maxDiff := make([][]int, n)
	for i := range maxDiff {
		maxDiff[i] = make([]int, m)
		for j := range maxDiff[i] {
			maxDiff[i][j] = math.MaxInt64
		}
	}
	maxDiff[0][0] = 0
	h := &hp{{}}
	for {
		p := heap.Pop(h).(point)
		if p.x == n-1 && p.y == m-1 {
			return p.maxDiff
		}
		if maxDiff[p.x][p.y] < p.maxDiff {
			continue
		}
		for _, d := range dir4 {
			if x, y := p.x+d.x, p.y+d.y; 0 <= x && x < n && 0 <= y && y < m {
				if diff := max(p.maxDiff, abs(heights[x][y]-heights[p.x][p.y])); diff < maxDiff[x][y] {
					maxDiff[x][y] = diff
					heap.Push(h, point{x, y, diff})
				}
			}
		}
	}
}

func main() {
	tests := []struct {
		heights [][]int
		ans     int
	}{
		{[][]int{{1, 2, 2}, {3, 8, 2}, {5, 3, 5}}, 2},
		{[][]int{{1, 2, 3}, {3, 8, 4}, {5, 3, 5}}, 1},
		{[][]int{{1, 2, 1, 1, 1}, {1, 2, 1, 2, 1}, {1, 2, 1, 2, 1}, {1, 2, 1, 2, 1}, {1, 1, 1, 2, 1}}, 0},
	}
	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minimumEffortPath(test.heights), index)
	}
}

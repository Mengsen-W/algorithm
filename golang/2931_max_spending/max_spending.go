// Package main ...
package main

import (
	"container/heap"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxSpending(values [][]int) int64 {
	m, n := len(values), len(values[0])
	pq := &Heap{}
	for i := 0; i < m; i++ {
		heap.Push(pq, []int{values[i][n-1], i, n - 1})
	}
	ans := int64(0)
	for turn := 1; turn <= m*n; turn++ {
		top := heap.Pop(pq).([]int)
		val, i, j := top[0], top[1], top[2]
		ans += int64(val) * int64(turn)
		if j > 0 {
			heap.Push(pq, []int{values[i][j-1], i, j - 1})
		}
	}
	return ans
}

type Heap [][]int

func (h Heap) Len() int {
	return len(h)
}

func (h Heap) Less(i, j int) bool {
	if h[i][0] == h[j][0] {
		return h[i][1] < h[j][1]
	}
	return h[i][0] < h[j][0]
}

func (h Heap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *Heap) Push(x interface{}) {
	*h = append(*h, x.([]int))
}

func (h *Heap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	tests := []struct {
		values [][]int
		ans    int64
	}{
		{[][]int{{8, 5, 2}, {6, 4, 1}, {9, 7, 3}}, 285},
		{[][]int{{10, 8, 6, 4, 2}, {9, 7, 5, 3, 2}}, 386},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxSpending(test.values), index)
	}
}

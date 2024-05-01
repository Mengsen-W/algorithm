/*
 * @Date: 2024-05-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-01
 * @FilePath: /algorithm/golang/2462_total_cost/total_cost.go
 */

// Package main ...
package main

import (
	"container/heap"
	"testing"

	"github.com/stretchr/testify/assert"
)

func totalCost(costs []int, k int, candidates int) int64 {
	n := len(costs)
	h := &Heap{}
	left, right := candidates-1, n-candidates

	if left+1 < right {
		for i := 0; i <= left; i++ {
			heap.Push(h, []int{costs[i], i})
		}
		for i := right; i < n; i++ {
			heap.Push(h, []int{costs[i], i})
		}
	} else {
		for i := 0; i < n; i++ {
			heap.Push(h, []int{costs[i], i})
		}
	}

	ans := int64(0)
	for i := 0; i < k; i++ {
		p := heap.Pop(h).([]int)
		cost, id := p[0], p[1]
		ans += int64(cost)
		if left+1 < right {
			if id <= left {
				left++
				heap.Push(h, []int{costs[left], left})
			} else {
				right--
				heap.Push(h, []int{costs[right], right})
			}
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
		costs      []int
		k          int
		candidates int
		ans        int64
	}{
		{[]int{17, 12, 10, 2, 7, 2, 11, 20, 8}, 3, 4, 11},
		{[]int{1, 2, 4, 1}, 3, 3, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, totalCost(test.costs, test.k, test.candidates), index)
	}
}

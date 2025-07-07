// Package main ...
package main

import (
	"container/heap"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func maxEvents(events [][]int) int {
	n := len(events)
	maxDay := 0
	for _, event := range events {
		if event[1] > maxDay {
			maxDay = event[1]
		}
	}
	sort.Slice(events, func(i, j int) bool {
		return events[i][0] < events[j][0]
	})
	pq := &IntHeap{}
	heap.Init(pq)
	ans := 0
	for i, j := 1, 0; i <= maxDay; i++ {
		for j < n && events[j][0] <= i {
			heap.Push(pq, events[j][1])
			j++
		}
		for pq.Len() > 0 && (*pq)[0] < i {
			heap.Pop(pq)
		}
		if pq.Len() > 0 {
			heap.Pop(pq)
			ans++
		}
	}
	return ans
}

type IntHeap []int

func (h IntHeap) Len() int {
	return len(h)
}

func (h IntHeap) Less(i, j int) bool {
	return h[i] < h[j]
}

func (h IntHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *IntHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	tests := []struct {
		events [][]int
		ans    int
	}{
		{[][]int{{1, 2}, {2, 3}, {3, 4}}, 3},
		{[][]int{{1, 2}, {2, 3}, {3, 4}, {1, 2}}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, maxEvents(test.events), index)
	}
}

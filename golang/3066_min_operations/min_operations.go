// Package main ...
package main

import (
	"container/heap"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minOperations(nums []int, k int) int {
	res := 0
	pq := &MinHeap{}
	heap.Init(pq)
	for _, num := range nums {
		heap.Push(pq, num)
	}

	for (*pq)[0] < k {
		x := heap.Pop(pq).(int)
		y := heap.Pop(pq).(int)
		heap.Push(pq, x+x+y)
		res++
	}

	return res
}

// MinHeap
type MinHeap []int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}

func (h *MinHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{2, 11, 10, 1, 3}, 10, 2},
		{[]int{1, 1, 2, 4, 9}, 20, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minOperations(test.nums, test.k), index)
	}
}

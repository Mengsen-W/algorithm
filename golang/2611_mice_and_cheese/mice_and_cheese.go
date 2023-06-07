/*
 * @Date: 2023-06-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-07
 * @FilePath: /algorithm/golang/2611_mice_and_cheese/mice_and_cheese.go
 */

// Package main ...
package main

import "container/heap"

type IntHeap []int

func (h IntHeap) Less(i, j int) bool {
	return h[i] < h[j]
}
func (h IntHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}
func (h *IntHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}
func (h IntHeap) Len() int {
	return len(h)
}
func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func miceAndCheese(reward1 []int, reward2 []int, k int) int {
	ans := 0
	n := len(reward1)
	pq := &IntHeap{}
	heap.Init(pq)
	for i := 0; i < n; i++ {
		ans += reward2[i]
		diff := reward1[i] - reward2[i]
		heap.Push(pq, diff)
		if pq.Len() > k {
			heap.Pop(pq)
		}
	}
	for pq.Len() > 0 {
		ans += heap.Pop(pq).(int)
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		reward1 := []int{1, 1, 3, 4}
		reward2 := []int{4, 4, 1, 1}
		k := 2
		ans := 15
		assert(miceAndCheese(reward1, reward2, k) == ans)
	}

	{
		reward1 := []int{1, 1}
		reward2 := []int{1, 1}
		k := 2
		ans := 2
		assert(miceAndCheese(reward1, reward2, k) == ans)
	}
}

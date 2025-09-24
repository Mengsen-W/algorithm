// Package main ...
package main

import "container/heap"

type NumberContainers struct {
	nums  map[int]int
	heaps map[int]*MinHeap
}

func Constructor() NumberContainers {
	return NumberContainers{
		nums:  make(map[int]int),
		heaps: make(map[int]*MinHeap),
	}
}

func (nc *NumberContainers) Change(index int, number int) {
	nc.nums[index] = number
	if _, exists := nc.heaps[number]; !exists {
		nc.heaps[number] = &MinHeap{}
		heap.Init(nc.heaps[number])
	}
	heap.Push(nc.heaps[number], index)
}

func (nc *NumberContainers) Find(number int) int {
	h, ok := nc.heaps[number]
	if !ok {
		return -1
	}
	for h.Len() > 0 && nc.nums[(*h)[0]] != number {
		heap.Pop(h)
	}
	if h.Len() == 0 {
		return -1
	}
	return (*h)[0]
}

type MinHeap []int

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *MinHeap) Push(x any) {
	*h = append(*h, x.(int))
}

func (h *MinHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	nc := Constructor()
	nc.Find(10)      // 没有数字 10 ，所以返回 -1 。
	nc.Change(2, 10) // 容器中下标为 2 处填入数字 10 。
	nc.Change(1, 10) // 容器中下标为 1 处填入数字 10 。
	nc.Change(3, 10) // 容器中下标为 3 处填入数字 10 。
	nc.Change(5, 10) // 容器中下标为 5 处填入数字 10 。
	nc.Find(10)      // 数字 10 所在的下标为 1 ，2 ，3 和 5 。因为最小下标为 1 ，所以返回 1 。
	nc.Change(1, 20) // 容器中下标为 1 处填入数字 20 。注意，下标 1 处之前为 10 ，现在被替换为 20 。
	nc.Find(10)      // 数字 10 所在下标为 2 ，3 和 5 。最小下标为 2 ，所以返回 2 。
}

// Package main ...
package main

import "container/heap"

type SeatManager struct {
	available *Heap
}

func Constructor(n int) SeatManager {
	h := &Heap{}
	heap.Init(h)
	for i := 1; i <= n; i++ {
		heap.Push(h, i)
	}
	return SeatManager{available: h}
}

func (this *SeatManager) Reserve() int {
	return heap.Pop(this.available).(int)
}

func (this *SeatManager) Unreserve(seatNumber int) {
	heap.Push(this.available, seatNumber)
}

type Heap []int

func (h Heap) Len() int           { return len(h) }
func (h Heap) Less(i, j int) bool { return h[i] < h[j] }
func (h Heap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *Heap) Push(x interface{}) {
	*h = append(*h, x.(int))
}

func (h *Heap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	seatManager := Constructor(5) // 初始化 SeatManager ，有 5 个座位。
	seatManager.Reserve()         // 所有座位都可以预约，所以返回最小编号的座位，也就是 1 。
	seatManager.Reserve()         // 可以预约的座位为 [2,3,4,5] ，返回最小编号的座位，也就是 2 。
	seatManager.Unreserve(2)      // 将座位 2 变为可以预约，现在可预约的座位为 [2,3,4,5] 。
	seatManager.Reserve()         // 可以预约的座位为 [2,3,4,5] ，返回最小编号的座位，也就是 2 。
	seatManager.Reserve()         // 可以预约的座位为 [3,4,5] ，返回最小编号的座位，也就是 3 。
	seatManager.Reserve()         // 可以预约的座位为 [4,5] ，返回最小编号的座位，也就是 4 。
	seatManager.Reserve()         // 唯一可以预约的是座位 5 ，所以返回 5 。
	seatManager.Unreserve(5)      // 将座位 5 变为可以预约，现在可预约的座位为 [5] 。
}

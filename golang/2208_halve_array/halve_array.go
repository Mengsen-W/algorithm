/*
 * @Date: 2023-07-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-25
 * @FilePath: /algorithm/golang/2208_halve_array/halve_array.go
 */

// Package main ...
package main

import (
	"container/heap"
	"testing"

	"github.com/stretchr/testify/assert"
)

type PriorityQueue []float64

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i] > pq[j]
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x any) {
	*pq = append(*pq, x.(float64))
}

func (pq *PriorityQueue) Pop() any {
	old, n := *pq, len(*pq)
	x := old[n-1]
	*pq = old[0 : n-1]
	return x
}

func halveArray(nums []int) int {
	pq := &PriorityQueue{}
	sum, sum2 := 0.0, 0.0
	for _, x := range nums {
		heap.Push(pq, float64(x))
		sum += float64(x)
	}
	res := 0
	for sum2 < sum/2 {
		x := heap.Pop(pq).(float64)
		sum2 += x / 2
		heap.Push(pq, x/2)
		res++
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{5, 19, 8, 1}, 3},
		{[]int{3, 8, 20}, 3},
	}

	for _, item := range tests {
		assert.Equal(&testing.T{}, halveArray(item.nums), item.ans)
	}
}

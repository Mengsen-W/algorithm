/*
 * @Date: 2023-12-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-23
 * @FilePath: /algorithm/golang/1962_min_stone_sum/min_stone_sum.go
 */

// Package main ...
package main

import (
	"container/heap"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

type PriorityQueue struct {
	sort.IntSlice
}

func (pq *PriorityQueue) Less(i, j int) bool {
	return pq.IntSlice[i] > pq.IntSlice[j]
}

func (pq *PriorityQueue) Push(v interface{}) {
	pq.IntSlice = append(pq.IntSlice, v.(int))
}

func (pq *PriorityQueue) Pop() interface{} {
	arr := pq.IntSlice
	v := arr[len(arr)-1]
	pq.IntSlice = arr[:len(arr)-1]
	return v
}

func minStoneSum(piles []int, k int) int {
	pq := &PriorityQueue{piles}
	heap.Init(pq)
	for i := 0; i < k; i++ {
		pile := heap.Pop(pq).(int)
		pile -= pile / 2
		heap.Push(pq, pile)
	}
	sum := 0
	for len(pq.IntSlice) > 0 {
		sum += heap.Pop(pq).(int)
	}
	return sum
}

func main() {
	tests := []struct {
		piles []int
		k     int
		ans   int
	}{
		{[]int{5, 4, 9}, 2, 12},
		{[]int{4, 3, 6, 7}, 3, 12},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minStoneSum(test.piles, test.k), index)
	}
}

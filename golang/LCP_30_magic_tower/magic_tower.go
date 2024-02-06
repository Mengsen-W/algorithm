/*
 * @Date: 2024-02-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-06
 * @FilePath: /algorithm/golang/LCP_30_magic_tower/magic_tower.go
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
	return pq.IntSlice[i] < pq.IntSlice[j]
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

func magicTower(nums []int) int {
	q := &PriorityQueue{}
	ans, hp, delay := 0, int64(1), int64(0)
	for _, num := range nums {
		if num < 0 {
			heap.Push(q, num)
		}
		hp += int64(num)
		if hp <= 0 {
			ans++
			delay += int64(q.IntSlice[0])
			hp -= int64(heap.Pop(q).(int))
		}
	}
	if hp+delay <= 0 {
		return -1
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{100, 100, 100, -250, -60, -140, -50, -50, 100, 150}, 1},
		{[]int{-200, -300, 400, 0}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, magicTower(test.nums), index)
	}
}

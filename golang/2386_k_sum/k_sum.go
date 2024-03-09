/*
 * @Date: 2024-03-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-09
 * @FilePath: /algorithm/golang/2386_k_sum/k_sum.go
 */

package main

import (
	"container/heap"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

type PriorityQueue [][2]int64

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i][0] < pq[j][0]
}

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x any) {
	(*pq) = append(*pq, x.([2]int64))
}

func (pq *PriorityQueue) Pop() any {
	n := len(*pq)
	x := (*pq)[n-1]
	(*pq) = (*pq)[:n-1]
	return x
}

func kSum(nums []int, k int) int64 {
	n, total := len(nums), int64(0)
	for i := range nums {
		if nums[i] >= 0 {
			total += int64(nums[i])
		} else {
			nums[i] = -nums[i]
		}
	}
	sort.Ints(nums)

	ret := int64(0)
	pq := PriorityQueue{
		[2]int64{int64(nums[0]), 0},
	}
	for j := 2; j <= k; j++ {
		t, i := pq[0][0], pq[0][1]
		heap.Pop(&pq)
		ret = t
		if i == int64(n-1) {
			continue
		}
		heap.Push(&pq, [2]int64{t + int64(nums[i+1]), i + 1})
		heap.Push(&pq, [2]int64{t - int64(nums[i]-nums[i+1]), i + 1})
	}
	return total - ret
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int64
	}{
		{[]int{2, 4, -2}, 5, 2},
		{[]int{1, -2, 3, 4, -10, 12}, 16, 10},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, kSum(test.nums, test.k), index)
	}
}

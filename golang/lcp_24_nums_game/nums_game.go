/*
 * @Date: 2024-02-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-01
 * @FilePath: /algorithm/golang/lcp_24_nums_game/nums_game.go
 */

// Package main ...
package main

import (
	"container/heap"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

type BasePQ struct {
	sort.IntSlice
}

func (pq *BasePQ) Push(x any) {
	pq.IntSlice = append(pq.IntSlice, x.(int))
}

func (pq *BasePQ) Pop() any {
	n := len(pq.IntSlice)
	x := pq.IntSlice[n-1]
	pq.IntSlice = pq.IntSlice[:n-1]
	return x
}

func (pq *BasePQ) Top() int {
	return pq.IntSlice[0]
}

type MinPQ struct {
	*BasePQ
}

func (pq *MinPQ) Less(i, j int) bool {
	return pq.BasePQ.Less(i, j)
}

type MaxPQ struct {
	*BasePQ
}

func (pq *MaxPQ) Less(i, j int) bool {
	return pq.BasePQ.Less(j, i)
}

func numsGame(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	lower, upper := &MaxPQ{&BasePQ{}}, &MinPQ{&BasePQ{}}
	lowerSum, upperSum := int64(0), int64(0)
	mod := int64(1e9 + 7)
	for i := 0; i < n; i++ {
		x := nums[i] - i
		if lower.Len() == 0 || lower.Top() >= x {
			lowerSum += int64(x)
			heap.Push(lower, x)
			if lower.Len() > upper.Len()+1 {
				upperSum += int64(lower.Top())
				heap.Push(upper, lower.Top())
				lowerSum -= int64(heap.Pop(lower).(int))
			}
		} else {
			upperSum += int64(x)
			heap.Push(upper, x)
			if lower.Len() < upper.Len() {
				lowerSum += int64(upper.Top())
				heap.Push(lower, upper.Top())
				upperSum -= int64(heap.Pop(upper).(int))
			}
		}
		if (i+1)%2 == 0 {
			res[i] = int((upperSum - lowerSum) % mod)
		} else {
			res[i] = int((upperSum - lowerSum + int64(lower.Top())) % mod)
		}
	}
	return res
}

func main() {
	tests := []struct {
		nums []int
		ans  []int
	}{
		{[]int{3, 4, 5, 1, 6, 7}, []int{0, 0, 0, 5, 6, 7}},
		{[]int{1, 2, 3, 4, 5}, []int{0, 0, 0, 0, 0}},
		{[]int{1, 1, 1, 2, 3, 4}, []int{0, 1, 2, 3, 3, 3}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, numsGame(test.nums), index)
	}
}

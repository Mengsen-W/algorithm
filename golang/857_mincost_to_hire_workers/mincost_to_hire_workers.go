/*
 * @Date: 2022-09-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-02
 * @FilePath: /algorithm/golang/857_mincost_to_hire_workers/mincost_to_hire_workers.go
 */

package main

import (
	"container/heap"
	"fmt"
	"math"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func mincostToHireWorkers(quality, wage []int, k int) float64 {
	n := len(quality)
	h := make([]int, n)
	for i := range h {
		h[i] = i
	}
	sort.Slice(h, func(i, j int) bool {
		a, b := h[i], h[j]
		return quality[a]*wage[b] > quality[b]*wage[a]
	})
	totalq := 0
	q := hp{}
	for i := 0; i < k-1; i++ {
		totalq += quality[h[i]]
		heap.Push(&q, quality[h[i]])
	}
	ans := 1e9
	for i := k - 1; i < n; i++ {
		idx := h[i]
		totalq += quality[idx]
		heap.Push(&q, quality[idx])
		ans = math.Min(ans, float64(wage[idx])/float64(quality[idx])*float64(totalq))
		totalq -= heap.Pop(&q).(int)
	}
	return ans
}

type hp struct{ sort.IntSlice }

func (h hp) Less(i, j int) bool  { return h.IntSlice[i] > h.IntSlice[j] }
func (h *hp) Push(v interface{}) { h.IntSlice = append(h.IntSlice, v.(int)) }
func (h *hp) Pop() interface{} {
	a := h.IntSlice
	v := a[len(a)-1]
	h.IntSlice = a[:len(a)-1]
	return v
}

func main() {
	tests := []struct {
		quality []int
		wage    []int
		k       int
		ans     float64
	}{
		{[]int{10, 20, 5}, []int{70, 50, 30}, 2, 105.00000},
		{[]int{3, 1, 10, 10, 1}, []int{4, 8, 2, 2, 7}, 3, 30.6666666666666666666667},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, mincostToHireWorkers(test.quality, test.wage, test.k), fmt.Sprintf("%f %d", mincostToHireWorkers(test.quality, test.wage, test.k), index))
	}
}

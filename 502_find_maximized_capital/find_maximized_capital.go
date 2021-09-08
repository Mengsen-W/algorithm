/*
 * @Date: 2021-09-08 09:31:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-08 10:47:23
 */

package main

import (
	"container/heap"
	"sort"
)

func findMaximizedCapital(k, w int, profits, capital []int) int {
	n := len(profits)
	type pair struct{ c, p int }
	arr := make([]pair, n)
	for i, p := range profits {
		arr[i] = pair{capital[i], p}
	}
	sort.Slice(arr, func(i, j int) bool { return arr[i].c < arr[j].c })

	h := &hp{}
	for cur := 0; k > 0; k-- {
		for cur < n && arr[cur].c <= w {
			heap.Push(h, arr[cur].p)
			cur++
		}
		if h.Len() == 0 {
			break
		}
		w += heap.Pop(h).(int)
	}
	return w
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
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		k := 2
		w := 0
		profits := []int{1, 2, 3}
		capital := []int{0, 1, 1}
		ans := 4
		assert(findMaximizedCapital(k, w, profits, capital) == ans)
	}
	{
		k := 3
		w := 0
		profits := []int{1, 2, 3}
		capital := []int{0, 1, 2}
		ans := 6
		assert(findMaximizedCapital(k, w, profits, capital) == ans)
	}
}

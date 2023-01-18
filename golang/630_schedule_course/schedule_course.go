/*
 * @Date: 2021-12-14 05:54:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-14 05:56:14
 */

package main

import (
	"container/heap"
	"sort"
)

func scheduleCourse(courses [][]int) int {
	sort.Slice(courses, func(i, j int) bool {
		return courses[i][1] < courses[j][1]
	})

	h := &Heap{}
	total := 0 // 优先队列中所有课程的总时间
	for _, course := range courses {
		if t := course[0]; total+t <= course[1] {
			total += t
			heap.Push(h, t)
		} else if h.Len() > 0 && t < h.IntSlice[0] {
			total += t - h.IntSlice[0]
			h.IntSlice[0] = t
			heap.Fix(h, 0)
		}
	}
	return h.Len()
}

type Heap struct {
	sort.IntSlice
}

func (h Heap) Less(i, j int) bool {
	return h.IntSlice[i] > h.IntSlice[j]
}

func (h *Heap) Push(x interface{}) {
	h.IntSlice = append(h.IntSlice, x.(int))
}

func (h *Heap) Pop() interface{} {
	a := h.IntSlice
	x := a[len(a)-1]
	h.IntSlice = a[:len(a)-1]
	return x
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}

	assert(scheduleCourse([][]int{
		{100, 200}, {200, 1300}, {1000, 1250}, {2000, 3200}}), 3)
	assert(scheduleCourse([][]int{{1, 2}}), 1)
	assert(scheduleCourse([][]int{{3, 2}, {4, 3}}), 0)
}

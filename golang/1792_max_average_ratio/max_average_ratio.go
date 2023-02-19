/*
 * @Date: 2023-02-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-19
 * @FilePath: /algorithm/golang/1792_max_average_ratio/max_average_ratio.go
 */

package main

import (
	"container/heap"
)

func maxAverageRatio(classes [][]int, extraStudents int) (ans float64) {
	h := hp(classes)
	heap.Init(&h)
	for ; extraStudents > 0; extraStudents-- {
		h[0][0]++
		h[0][1]++
		heap.Fix(&h, 0)
	}
	for _, c := range h {
		ans += float64(c[0]) / float64(c[1])
	}
	return ans / float64(len(classes))
}

type hp [][]int

func (h hp) Len() int { return len(h) }
func (h hp) Less(i, j int) bool {
	a, b := h[i], h[j]
	return (a[1]-a[0])*b[1]*(b[1]+1) > (b[1]-b[0])*a[1]*(a[1]+1)
}
func (h hp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (hp) Push(interface{})     {}
func (hp) Pop() (_ interface{}) { return }

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		classes := [][]int{{1, 2}, {3, 5}, {2, 2}}
		extraStudents := 2
		ans := 0.7833
		assert(maxAverageRatio(classes, extraStudents) == ans)
	}

	{
		classes := [][]int{{2, 4}, {3, 9}, {4, 5}, {2, 10}}
		extraStudents := 4
		ans := 0.53485
		assert(maxAverageRatio(classes, extraStudents) == ans)
	}
}

/*
 * @Date: 2021-12-24 01:16:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-24 01:19:22
 */

package main

import "container/heap"

func eatenApples(apples, days []int) (ans int) {
	h := hp{}
	i := 0
	for ; i < len(apples); i++ {
		for len(h) > 0 && h[0].end <= i {
			heap.Pop(&h)
		}
		if apples[i] > 0 {
			heap.Push(&h, pair{i + days[i], apples[i]})
		}
		if len(h) > 0 {
			h[0].left--
			if h[0].left == 0 {
				heap.Pop(&h)
			}
			ans++
		}
	}
	for len(h) > 0 {
		for len(h) > 0 && h[0].end <= i {
			heap.Pop(&h)
		}
		if len(h) == 0 {
			break
		}
		p := heap.Pop(&h).(pair)
		num := min(p.end-i, p.left)
		ans += num
		i += num
	}
	return
}

type pair struct{ end, left int }
type hp []pair

func (h hp) Len() int            { return len(h) }
func (h hp) Less(i, j int) bool  { return h[i].end < h[j].end }
func (h hp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{}) { *h = append(*h, v.(pair)) }
func (h *hp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}
	assert(eatenApples([]int{1, 2, 3, 5, 2}, []int{3, 2, 1, 4, 2}), 7)
	assert(eatenApples([]int{3, 0, 0, 0, 0, 2}, []int{3, 0, 0, 0, 0, 2}), 7)
}

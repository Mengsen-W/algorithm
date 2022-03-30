/*
 * @Date: 2022-03-29 23:54:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-30 00:14:29
 * @FilePath: /algorithm/1066_busiest_servers/busiest_servers.go
 */

package main

import (
	"container/heap"
	"reflect"
	"sort"
)

func busiestServers(k int, arrival, load []int) (ans []int) {
	available := hi{make([]int, k)}
	for i := 0; i < k; i++ {
		available.IntSlice[i] = i
	}
	busy := hp{}
	requests := make([]int, k)
	maxRequest := 0
	for i, t := range arrival {
		for len(busy) > 0 && busy[0].end <= t {
			heap.Push(&available, i+((busy[0].id-i)%k+k)%k) // 保证得到的是一个不小于 i 的且与 id 同余的数
			heap.Pop(&busy)
		}
		if available.Len() == 0 {
			continue
		}
		id := heap.Pop(&available).(int) % k
		requests[id]++
		if requests[id] > maxRequest {
			maxRequest = requests[id]
			ans = []int{id}
		} else if requests[id] == maxRequest {
			ans = append(ans, id)
		}
		heap.Push(&busy, pair{t + load[i], id})
	}
	return
}

type hi struct{ sort.IntSlice }

func (h *hi) Push(v interface{}) { h.IntSlice = append(h.IntSlice, v.(int)) }
func (h *hi) Pop() interface{} {
	a := h.IntSlice
	v := a[len(a)-1]
	h.IntSlice = a[:len(a)-1]
	return v
}

type pair struct{ end, id int }
type hp []pair

func (h hp) Len() int            { return len(h) }
func (h hp) Less(i, j int) bool  { return h[i].end < h[j].end }
func (h hp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{}) { *h = append(*h, v.(pair)) }
func (h *hp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		k := 3
		arrival := []int{1, 2, 3, 4, 5}
		load := []int{5, 2, 3, 3, 3}
		ans := []int{1}
		assert(busiestServers(k, arrival, load), ans)
	}

	{
		k := 3
		arrival := []int{1, 2, 3, 4}
		load := []int{1, 2, 1, 2}
		ans := []int{0}
		assert(busiestServers(k, arrival, load), ans)
	}

	{
		k := 3
		arrival := []int{1, 2, 3}
		load := []int{10, 12, 11}
		ans := []int{0, 1, 2}
		assert(busiestServers(k, arrival, load), ans)
	}

	{
		k := 3
		arrival := []int{1, 2, 3, 4, 8, 9, 10}
		load := []int{5, 2, 10, 3, 1, 2, 2}
		ans := []int{1}
		assert(busiestServers(k, arrival, load), ans)
	}

	{
		k := 3
		arrival := []int{1}
		load := []int{1}
		ans := []int{0}
		assert(busiestServers(k, arrival, load), ans)
	}
}

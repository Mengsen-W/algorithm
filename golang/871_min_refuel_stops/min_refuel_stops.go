// Package main ...
package main

import (
	"container/heap"
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minRefuelStops(target, startFuel int, stations [][]int) (ans int) {
	fuel, prev, h := startFuel, 0, hp{}
	for i, n := 0, len(stations); i <= n; i++ {
		curr := target
		if i < n {
			curr = stations[i][0]
		}
		fuel -= curr - prev
		for fuel < 0 && h.Len() > 0 {
			fuel += heap.Pop(&h).(int)
			ans++
		}
		if fuel < 0 {
			return -1
		}
		if i < n {
			heap.Push(&h, stations[i][1])
			prev = curr
		}
	}
	return
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
		target    int
		startFuel int
		stations  [][]int
		ans       int
	}{
		{1, 1, [][]int{}, 0},
		{100, 1, [][]int{{10, 100}}, -1},
		{100, 10, [][]int{{10, 60}, {20, 30}, {30, 30}, {60, 40}}, 2},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minRefuelStops(test.target, test.startFuel, test.stations), index)
	}
}

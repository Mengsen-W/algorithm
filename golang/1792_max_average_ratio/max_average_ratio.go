// Package main ...
package main

import (
	"container/heap"
	"testing"

	"github.com/stretchr/testify/assert"
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
func (h hp) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (hp) Push(any)        {}
func (hp) Pop() (_ any)    { return }

func main() {
	tests := []struct {
		classes       [][]int
		extraStudents int
		ans           float64
	}{
		{[][]int{{1, 2}, {3, 5}, {2, 2}}, 2, 0.78333},
		{[][]int{{2, 4}, {3, 9}, {4, 5}, {2, 10}}, 4, 0.53485},
	}
	for index, test := range tests {
		res := maxAverageRatio(test.classes, test.extraStudents)
		assert.InDelta(&testing.T{}, test.ans, res, 0.00001, index)
	}
}

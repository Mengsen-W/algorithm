/*
 * @Date: 2023-10-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-28
 * @FilePath: /algorithm/golang/2558_pick_gifts/pick_gifts.go
 */

// Package main ...
package main

import (
	"container/heap"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func pickGifts(gifts []int, k int) int64 {
	h := &hp{}
	for _, g := range gifts {
		h.push(g)
	}
	for i := 0; i < k; i++ {
		t := h.pop()
		t = int(math.Sqrt(float64(t)))
		h.push(t)
	}
	var res int
	for h.Len() > 0 {
		res += h.pop()
	}
	return int64(res)
}

type hp []int

func (h hp) Len() int              { return len(h) }
func (h hp) Less(i, j int) bool    { return h[i] > h[j] }
func (h hp) Swap(i, j int)         { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{})   { *h = append(*h, v.(int)) }
func (h *hp) Pop() (v interface{}) { a := *h; *h, v = a[:len(a)-1], a[len(a)-1]; return }
func (h *hp) push(v int)           { heap.Push(h, v) }
func (h *hp) pop() int             { return heap.Pop(h).(int) }
func (h *hp) top() int             { a := *h; return a[0] }

func main() {
	tests := []struct {
		gifts []int
		k     int
		ans   int64
	}{
		{[]int{25, 64, 9, 4, 100}, 4, 29},
		{[]int{1, 1, 1, 1}, 4, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, pickGifts(test.gifts, test.k), index)
	}
}

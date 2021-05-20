/*
 * @Date: 2021-05-20 08:49:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-20 09:30:12
 */

package main

import (
	"container/heap"
)

type pair struct {
	w string
	c int
}
type hp []pair

func (h hp) Len() int            { return len(h) }
func (h hp) Less(i, j int) bool  { a, b := h[i], h[j]; return a.c < b.c || a.c == b.c && a.w > b.w }
func (h hp) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v interface{}) { *h = append(*h, v.(pair)) }
func (h *hp) Pop() interface{}   { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }

func topKFrequent(words []string, k int) []string {
	cnt := map[string]int{}
	for _, w := range words {
		cnt[w]++
	}
	h := &hp{}
	for w, c := range cnt {
		heap.Push(h, pair{w, c})
		if h.Len() > k {
			heap.Pop(h)
		}
	}
	ans := make([]string, k)
	for i := k - 1; i >= 0; i-- {
		ans[i] = heap.Pop(h).(pair).w
	}
	return ans
}

func main() {
	assert_string_slice := func(words, ans []string) {
		size := len(words)
		if size != len(ans) {
			panic("Not Passed!")
		}
		for i := 1; i < size; i++ {
			if words[i] != ans[i] {
				panic("Not Passed!")
			}
		}
	}

	assert_string_slice(topKFrequent(
		[]string{"i", "love", "leetcode", "i", "love", "coding"}, 2),
		[]string{"i", "love"})
	assert_string_slice(topKFrequent(
		[]string{"the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"}, 4),
		[]string{"the", "is", "sunny", "day"})
}

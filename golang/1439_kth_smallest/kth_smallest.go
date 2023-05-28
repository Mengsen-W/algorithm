/*
 * @Date: 2023-05-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-28
 * @FilePath: /algorithm/golang/1439_kth_smallest/kth_smallest.go
 */

// Package main ...
package main

import (
	"container/heap"
)

func kSmallestPairs(nums1, nums2 []int, k int) []int {
	n, m := len(nums1), len(nums2)
	ans := make([]int, 0, min(k, n*m)) // 预分配空间
	h := hp{{nums1[0] + nums2[0], 0, 0}}
	for len(h) > 0 && len(ans) < k {
		p := heap.Pop(&h).(tuple)
		i, j := p.i, p.j
		ans = append(ans, nums1[i]+nums2[j]) // 数对和
		if j == 0 && i+1 < n {
			heap.Push(&h, tuple{nums1[i+1] + nums2[0], i + 1, 0})
		}
		if j+1 < m {
			heap.Push(&h, tuple{nums1[i] + nums2[j+1], i, j + 1})
		}
	}
	return ans
}

func kthSmallest(mat [][]int, k int) int {
	a := []int{0}
	for _, row := range mat {
		a = kSmallestPairs(row, a, k)
	}
	return a[k-1]
}

type tuple struct{ s, i, j int }
type hp []tuple

func (h hp) Len() int           { return len(h) }
func (h hp) Less(i, j int) bool { return h[i].s < h[j].s }
func (h hp) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *hp) Push(v any)        { *h = append(*h, v.(tuple)) }
func (h *hp) Pop() any          { a := *h; v := a[len(a)-1]; *h = a[:len(a)-1]; return v }
func min(a, b int) int {
	if b < a {
		return b
	}
	return a
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		mat := [][]int{{1, 3, 11}, {2, 4, 6}}
		k := 5
		ans := 7
		assert(kthSmallest(mat, k) == ans)
	}

	{
		mat := [][]int{{1, 3, 11}, {2, 4, 6}}
		k := 9
		ans := 17
		assert(kthSmallest(mat, k) == ans)
	}

	{
		mat := [][]int{{1, 10, 10}, {1, 4, 5}, {2, 3, 6}}
		k := 7
		ans := 9
		assert(kthSmallest(mat, k) == ans)
	}

	{
		mat := [][]int{{1, 1, 10}, {2, 2, 9}}
		k := 7
		ans := 12
		assert(kthSmallest(mat, k) == ans)
	}
}

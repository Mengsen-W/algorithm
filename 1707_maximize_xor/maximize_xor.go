/*
 * @Date: 2021-05-24 09:54:59
 * @Author: mengsenwang
 * @LastEditors: mengsenwang
 * @LastEditTime: 2021-05-24 10:13:58
 */

package main

import "math"

const L = 30

type trie struct {
	children [2]*trie
	min      int
}

func (t *trie) insert(val int) {
	node := t
	if val < node.min {
		node.min = val
	}
	for i := L - 1; i >= 0; i-- {
		bit := val >> i & 1
		if node.children[bit] == nil {
			node.children[bit] = &trie{min: val}
		}
		node = node.children[bit]
		if val < node.min {
			node.min = val
		}
	}
}

func (t *trie) getMaxXorWithLimit(val, limit int) (ans int) {
	node := t
	if node.min > limit {
		return -1
	}
	for i := L - 1; i >= 0; i-- {
		bit := val >> i & 1
		if node.children[bit^1] != nil && node.children[bit^1].min <= limit {
			ans |= 1 << i
			bit ^= 1
		}
		node = node.children[bit]
	}
	return
}

func maximizeXor(nums []int, queries [][]int) []int {
	t := &trie{min: math.MaxInt32}
	for _, val := range nums {
		t.insert(val)
	}
	ans := make([]int, len(queries))
	for i, q := range queries {
		ans[i] = t.getMaxXorWithLimit(q[0], q[1])
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	CompareSlice := func(a, b []int) bool {
		if len(a) != len(b) {
			return false
		}

		if (a == nil) != (b == nil) {
			return false
		}

		for key, value := range a {
			if value != b[key] {
				return false
			}
		}

		return true
	}

	{
		nums := []int{0, 1, 2, 3, 4}
		queries := [][]int{{3, 1}, {1, 3}, {5, 6}}
		ans := []int{3, 3, 7}
		assert(CompareSlice(maximizeXor(nums, queries), ans))
	}
	{
		nums := []int{5, 2, 4, 6, 6, 3}
		queries := [][]int{{12, 4}, {8, 1}, {6, 3}}
		ans := []int{15, -1, 5}
		assert(CompareSlice(maximizeXor(nums, queries), ans))
	}
}

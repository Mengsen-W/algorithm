/*
 * @Date: 2022-06-21 09:45:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-21 09:52:27
 * @FilePath: /algorithm/715_range_module/range_module.go
 */

package main

type RangeModule struct {
	*redblacktree.Tree
}

func Constructor() RangeModule {
	return RangeModule{redblacktree.NewWithIntComparator()}
}

func (t RangeModule) AddRange(left, right int) {
	if node, ok := t.Floor(left); ok {
		r := node.Value.(int)
		if r >= right {
			return
		}
		if r >= left {
			left = node.Key.(int)
			t.Remove(left)
		}
	}
	for node, ok := t.Ceiling(left); ok && node.Key.(int) <= right; node, ok = t.Ceiling(left) {
		right = max(right, node.Value.(int))
		t.Remove(node.Key)
	}
	t.Put(left, right)
}

func (t RangeModule) QueryRange(left, right int) bool {
	node, ok := t.Floor(left)
	return ok && node.Value.(int) >= right
}

func (t RangeModule) RemoveRange(left, right int) {
	if node, ok := t.Floor(left); ok {
		l, r := node.Key.(int), node.Value.(int)
		if r >= right {
			if l == left {
				t.Remove(l)
			} else {
				node.Value = left
			}
			if right != r {
				t.Put(right, r)
			}
			return
		}
		if r > left {
			node.Value = left
		}
	}
	for node, ok := t.Ceiling(left); ok && node.Key.(int) < right; node, ok = t.Ceiling(left) {
		r := node.Value.(int)
		t.Remove(node.Key)
		if r > right {
			t.Put(right, r)
			break
		}
	}
}

func max(a, b int) int {
	if b > a {
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
	r := Constructor()
	r.AddRange(10, 20)
	r.RemoveRange(14, 16)
	assert(r.QueryRange(10, 14) == true)
	assert(r.QueryRange(13, 15) == false)
	assert(r.QueryRange(16, 17) == true)
}

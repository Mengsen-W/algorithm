/*
 * @Date: 2023-11-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-12
 * @FilePath: /algorithm/golang/715_RangeModule/RangeModule.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/emirpasic/gods/trees/redblacktree"
	"github.com/stretchr/testify/assert"
)

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
			if l == left {
				t.Remove(l)
			} else {
				node.Value = left
			}
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
	t := &testing.T{}
	rangeModule := Constructor()
	rangeModule.AddRange(10, 20)
	rangeModule.RemoveRange(14, 16)
	assert.Equal(t, rangeModule.QueryRange(10, 14), true)  // 返回 true （区间 [10, 14) 中的每个数都正在被跟踪）
	assert.Equal(t, rangeModule.QueryRange(13, 15), false) // 返回 false（未跟踪区间 [13, 15) 中像 14, 14.03, 14.17 这样的数字）
	assert.Equal(t, rangeModule.QueryRange(16, 17), true)  // 返回 true （尽管执行了删除操作，区间 [16, 17) 中的数字 16 仍然会被跟踪
}

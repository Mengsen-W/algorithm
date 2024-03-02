/*
 * @Date: 2024-03-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-02
 * @FilePath: /algorithm/golang/2368_reachable_nodes/reachable_nodes.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type UnionFind struct {
	f    []int
	rank []int
}

func NewUnionFind(n int) *UnionFind {
	uf := &UnionFind{
		f:    make([]int, n),
		rank: make([]int, n),
	}
	for i := 0; i < n; i++ {
		uf.f[i] = i
	}
	return uf
}

func (uf *UnionFind) merge(x, y int) {
	rx := uf.find(x)
	ry := uf.find(y)
	if rx != ry {
		if uf.rank[rx] > uf.rank[ry] {
			uf.f[ry] = rx
		} else if uf.rank[rx] < uf.rank[ry] {
			uf.f[rx] = ry
		} else {
			uf.f[ry] = rx
			uf.rank[rx]++
		}
	}
}

func (uf *UnionFind) find(x int) int {
	if x != uf.f[x] {
		uf.f[x] = uf.find(uf.f[x])
	}
	return uf.f[x]
}

func (uf *UnionFind) count() int {
	cnt := 0
	rt := uf.find(0)
	for i := 0; i < len(uf.f); i++ {
		if rt == uf.find(i) {
			cnt++
		}
	}
	return cnt
}

func reachableNodes(n int, edges [][]int, restricted []int) int {
	isRestricted := make([]int, n)
	for _, x := range restricted {
		isRestricted[x] = 1
	}

	uf := NewUnionFind(n)
	for _, v := range edges {
		if isRestricted[v[0]] == 1 || isRestricted[v[1]] == 1 {
			continue
		}
		uf.merge(v[0], v[1])
	}
	return uf.count()
}

func main() {
	tests := []struct {
		n          int
		edges      [][]int
		restricted []int
		ans        int
	}{
		{7, [][]int{{0, 1}, {1, 2}, {3, 1}, {4, 0}, {0, 5}, {5, 6}}, []int{4, 5}, 4},
		{7, [][]int{{0, 1}, {0, 2}, {0, 5}, {0, 4}, {3, 2}, {6, 5}}, []int{4, 2, 1}, 3},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, reachableNodes(test.n, test.edges, test.restricted), index)
	}
}

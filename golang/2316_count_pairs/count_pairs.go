/*
 * @Date: 2023-10-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-21
 * @FilePath: /algorithm/golang/2316_count_pairs/count_pairs.go
 */

// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type UnionFind struct {
	parents []int
	sizes   []int
}

func NewUnionFind(n int) *UnionFind {
	parents, sizes := make([]int, n), make([]int, n)
	for i := 0; i < n; i++ {
		parents[i], sizes[i] = i, 1
	}
	return &UnionFind{
		parents: parents,
		sizes:   sizes,
	}
}

func (uf *UnionFind) Find(x int) int {
	if uf.parents[x] == x {
		return x
	}
	uf.parents[x] = uf.Find(uf.parents[x])
	return uf.parents[x]
}

func (uf *UnionFind) Union(x, y int) {
	rx, ry := uf.Find(x), uf.Find(y)
	if rx != ry {
		if uf.sizes[x] > uf.sizes[y] {
			uf.parents[ry], uf.sizes[rx] = rx, uf.sizes[rx]+uf.sizes[ry]
		} else {
			uf.parents[rx], uf.sizes[ry] = ry, uf.sizes[rx]+uf.sizes[ry]
		}
	}
}

func (uf *UnionFind) GetSize(x int) int {
	return uf.sizes[x]
}

func countPairs(n int, edges [][]int) int64 {
	uf := NewUnionFind(n)
	for _, edge := range edges {
		uf.Union(edge[0], edge[1])
	}
	var res int64
	for i := 0; i < n; i++ {
		res += int64(n - uf.GetSize(uf.Find(i)))
	}
	return res / 2
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		ans   int64
	}{
		{3, [][]int{{0, 1}, {0, 2}, {1, 2}}, 0},
		{7, [][]int{{0, 2}, {0, 5}, {2, 4}, {1, 6}, {5, 4}}, 14},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, countPairs(test.n, test.edges), index)
	}
}

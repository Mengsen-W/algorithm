// Package main ...
package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func smallestEquivalentString(s1 string, s2 string, baseStr string) string {
	uf := NewUnionFind(26)
	for i := 0; i < len(s1); i++ {
		uf.Unite(int(s1[i]-'a'), int(s2[i]-'a'))
	}

	res := []byte(baseStr)
	for i := range res {
		res[i] = byte('a' + uf.Find(int(res[i]-'a')))
	}
	return string(res)
}

type UnionFind struct {
	parent []int
}

func NewUnionFind(n int) *UnionFind {
	uf := &UnionFind{parent: make([]int, n)}
	for i := 0; i < n; i++ {
		uf.parent[i] = i
	}
	return uf
}

func (uf *UnionFind) Find(x int) int {
	if uf.parent[x] != x {
		uf.parent[x] = uf.Find(uf.parent[x])
	}
	return uf.parent[x]
}

func (uf *UnionFind) Unite(x, y int) {
	x, y = uf.Find(x), uf.Find(y)
	if x == y {
		return
	}
	if x > y {
		x, y = y, x
	}
	// 总是让字典序更小的作为集合代表字符
	uf.parent[y] = x
}

func main() {
	tests := []struct {
		s1      string
		s2      string
		baseStr string
		ans     string
	}{
		{"parker", "morris", "parser", "makkek"},
		{"hello", "world", "hold", "hdld"},
		{"leetcode", "programs", "sourcecode", "aauaaaaada"},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, smallestEquivalentString(test.s1, test.s2, test.baseStr), "testcase: %d", index)
	}
}

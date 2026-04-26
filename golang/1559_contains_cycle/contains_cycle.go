// Package main ...
package main

import "fmt"

func containsCycle(grid [][]byte) bool {
	m, n := len(grid), len(grid[0])
	uf := NewUnionFind(m * n)

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if i > 0 && grid[i][j] == grid[i-1][j] {
				if !uf.FindAndUnite(i*n+j, (i-1)*n+j) {
					return true
				}
			}
			if j > 0 && grid[i][j] == grid[i][j-1] {
				if !uf.FindAndUnite(i*n+j, i*n+j-1) {
					return true
				}
			}
		}
	}
	return false
}

type UnionFind struct {
	parent []int
	size   []int
}

func NewUnionFind(n int) *UnionFind {
	parent := make([]int, n)
	size := make([]int, n)
	for i := 0; i < n; i++ {
		parent[i] = i
		size[i] = 1
	}
	return &UnionFind{parent: parent, size: size}
}

func (uf *UnionFind) FindSet(x int) int {
	if uf.parent[x] != x {
		uf.parent[x] = uf.FindSet(uf.parent[x])
	}
	return uf.parent[x]
}

func (uf *UnionFind) Unite(x, y int) {
	if uf.size[x] < uf.size[y] {
		x, y = y, x
	}
	uf.parent[y] = x
	uf.size[x] += uf.size[y]
}

func (uf *UnionFind) FindAndUnite(x, y int) bool {
	parentX := uf.FindSet(x)
	parentY := uf.FindSet(y)
	if parentX != parentY {
		uf.Unite(parentX, parentY)
		return true
	}
	return false
}

func main() {
	tests := []struct {
		grid [][]byte
		ans  bool
	}{
		{[][]byte{{'a', 'a', 'a', 'a'}, {'a', 'b', 'b', 'a'}, {'a', 'b', 'b', 'a'}, {'a', 'a', 'a', 'a'}}, true},
		{[][]byte{{'c', 'c', 'c', 'a'}, {'c', 'd', 'c', 'c'}, {'c', 'c', 'e', 'c'}, {'f', 'c', 'c', 'c'}}, true},
		{[][]byte{{'a', 'b', 'b'}, {'b', 'z', 'b'}, {'b', 'b', 'a'}}, false},
	}

	for _, test := range tests {
		if ret := containsCycle(test.grid); ret != test.ans {
			fmt.Println("error:", test.grid, ret)
		}
	}
}

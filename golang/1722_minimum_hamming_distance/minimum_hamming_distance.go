// Package main ...
package main

import "fmt"

type UnionFind struct {
	fa   []int
	rank []int
}

func NewUnionFind(n int) *UnionFind {
	fa := make([]int, n)
	rank := make([]int, n)
	for i := 0; i < n; i++ {
		fa[i] = i
	}
	return &UnionFind{fa: fa, rank: rank}
}

func (uf *UnionFind) find(x int) int {
	if uf.fa[x] != x {
		uf.fa[x] = uf.find(uf.fa[x])
	}
	return uf.fa[x]
}

func (uf *UnionFind) union(x, y int) {
	x = uf.find(x)
	y = uf.find(y)
	if x == y {
		return
	}
	if uf.rank[x] < uf.rank[y] {
		x, y = y, x
	}
	uf.fa[y] = x
	if uf.rank[x] == uf.rank[y] {
		uf.rank[x]++
	}
}

func minimumHammingDistance(source []int, target []int, allowedSwaps [][]int) int {
	n := len(source)
	uf := NewUnionFind(n)
	for _, pair := range allowedSwaps {
		uf.union(pair[0], pair[1])
	}

	sets := make(map[int]map[int]int)
	for i := 0; i < n; i++ {
		f := uf.find(i)
		if sets[f] == nil {
			sets[f] = make(map[int]int)
		}
		sets[f][source[i]]++
	}

	ans := 0
	for i := 0; i < n; i++ {
		f := uf.find(i)
		if sets[f][target[i]] > 0 {
			sets[f][target[i]]--
		} else {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		source       []int
		target       []int
		allowedSwaps [][]int
		ans          int
	}{
		{[]int{1, 2, 3, 4}, []int{2, 1, 4, 5}, [][]int{{0, 1}, {2, 3}}, 1},
		{[]int{1, 2, 3, 4}, []int{1, 3, 2, 4}, [][]int{}, 2},
		{[]int{5, 1, 2, 4, 3}, []int{1, 5, 4, 2, 3}, [][]int{{0, 4}, {4, 2}, {1, 3}, {1, 4}}, 0},
	}

	for index, test := range tests {
		if minimumHammingDistance(test.source, test.target, test.allowedSwaps) != test.ans {
			fmt.Println(index)
		}
	}
}

// Package main ...
package main

import (
	"fmt"
)

type DSU struct {
	parent []int
	rank   []int
}

func NewDSU(n int) *DSU {
	parent := make([]int, n)
	rank := make([]int, n)
	for i := 0; i < n; i++ {
		parent[i] = i
		rank[i] = 1
	}
	return &DSU{parent: parent, rank: rank}
}

func (d *DSU) Find(x int) int {
	if d.parent[x] != x {
		d.parent[x] = d.Find(d.parent[x])
	}
	return d.parent[x]
}

func (d *DSU) Union(x, y int) {
	rx, ry := d.Find(x), d.Find(y)
	if rx == ry {
		return
	}
	if d.rank[rx] > d.rank[ry] {
		d.parent[ry] = rx
	} else if d.rank[ry] > d.rank[rx] {
		d.parent[rx] = ry
	} else {
		d.parent[rx] = ry
		d.rank[ry]++
	}
}

func countCompleteComponents(n int, edges [][]int) int {
	dsu := NewDSU(n)
	for _, e := range edges {
		dsu.Union(e[0], e[1])
	}

	numV := make([]int, n)
	numE := make([]int, n)
	for i := 0; i < n; i++ {
		numV[dsu.Find(i)]++
	}
	for _, e := range edges {
		numE[dsu.Find(e[0])]++
	}

	ans := 0
	for i := 0; i < n; i++ {
		if dsu.Find(i) == i && numE[i] == numV[i]*(numV[i]-1)/2 {
			ans++
		}
	}
	return ans
}

func main() {
	tests := []struct {
		n     int
		edges [][]int
		ans   int
	}{
		{6, [][]int{{0, 1}, {0, 2}, {1, 2}, {3, 4}}, 3},
		{6, [][]int{{0, 1}, {0, 2}, {1, 2}, {3, 4}, {3, 5}}, 1},
	}

	for index, test := range tests {
		if countCompleteComponents(test.n, test.edges) != test.ans {
			fmt.Println(index)
		}
	}
}

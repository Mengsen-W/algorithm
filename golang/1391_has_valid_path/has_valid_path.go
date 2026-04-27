// Package main ...
package main

import "fmt"

func hasValidPath(grid [][]int) bool {
	m := len(grid)
	n := len(grid[0])
	ds := NewDisjointSet(m, n)

	getId := func(x, y int) int {
		return x*n + y
	}

	detectL := func(x, y int) {
		if y-1 >= 0 && (grid[x][y-1] == 4 || grid[x][y-1] == 6 || grid[x][y-1] == 1) {
			ds.Merge(getId(x, y), getId(x, y-1))
		}
	}

	detectR := func(x, y int) {
		if y+1 < n && (grid[x][y+1] == 3 || grid[x][y+1] == 5 || grid[x][y+1] == 1) {
			ds.Merge(getId(x, y), getId(x, y+1))
		}
	}

	detectU := func(x, y int) {
		if x-1 >= 0 && (grid[x-1][y] == 3 || grid[x-1][y] == 4 || grid[x-1][y] == 2) {
			ds.Merge(getId(x, y), getId(x-1, y))
		}
	}

	detectD := func(x, y int) {
		if x+1 < m && (grid[x+1][y] == 5 || grid[x+1][y] == 6 || grid[x+1][y] == 2) {
			ds.Merge(getId(x, y), getId(x+1, y))
		}
	}

	handler := func(x, y int) {
		switch grid[x][y] {
		case 1:
			detectL(x, y)
			detectR(x, y)
		case 2:
			detectU(x, y)
			detectD(x, y)
		case 3:
			detectL(x, y)
			detectD(x, y)
		case 4:
			detectR(x, y)
			detectD(x, y)
		case 5:
			detectL(x, y)
			detectU(x, y)
		case 6:
			detectR(x, y)
			detectU(x, y)
		}
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			handler(i, j)
		}
	}

	return ds.Find(getId(0, 0)) == ds.Find(getId(m-1, n-1))
}

type DisjointSet struct {
	f []int
}

func NewDisjointSet(m, n int) *DisjointSet {
	size := m * n
	f := make([]int, size)
	for i := 0; i < size; i++ {
		f[i] = i
	}
	return &DisjointSet{f: f}
}

func (ds *DisjointSet) Find(x int) int {
	if x == ds.f[x] {
		return x
	}
	ds.f[x] = ds.Find(ds.f[x])
	return ds.f[x]
}

func (ds *DisjointSet) Merge(x, y int) {
	ds.f[ds.Find(x)] = ds.Find(y)
}

func main() {
	tests := []struct {
		grid [][]int
		ans  bool
	}{
		{[][]int{{2, 4, 3}, {6, 5, 2}}, true},
		{[][]int{{1, 2, 1}, {1, 2, 1}}, false},
		{[][]int{{1, 1, 2}}, false},
		{[][]int{{1, 1, 1, 1, 1, 1, 3}}, true},
		{[][]int{{2}, {2}, {2}, {2}, {2}, {2}, {6}}, true},
	}

	for _, test := range tests {
		if ret := hasValidPath(test.grid); ret != test.ans {
			fmt.Println(test.grid, ret, test.ans)
		}
	}
}

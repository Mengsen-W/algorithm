/*
 * @Date: 2022-12-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-19
 * @FilePath: /algorithm/1971_valid_path/valid_path.go
 */

package main

func validPath(n int, edges [][]int, source int, destination int) bool {
	if source == destination {
		return true
	}
	uf := NewUnionFind(n)
	for _, edge := range edges {
		uf.uin(edge[0], edge[1])
	}
	return uf.connect(source, destination)
}

type UnionFind struct {
	parent []int
	rank   []int
}

func NewUnionFind(n int) *UnionFind {
	parent := make([]int, n)
	rank := make([]int, n)
	for i := 0; i < n; i++ {
		parent[i] = i
	}
	return &UnionFind{
		parent: parent,
		rank:   rank,
	}
}

func (u *UnionFind) uin(x, y int) {
	rootx := u.find(x)
	rooty := u.find(y)
	if rootx != rooty {
		if u.rank[rootx] > u.rank[rooty] {
			u.parent[rooty] = rootx
		} else if u.rank[rootx] < u.rank[rooty] {
			u.parent[rootx] = rooty
		} else {
			u.parent[rooty] = rootx
			u.rank[rootx]++
		}
	}
}

func (u *UnionFind) find(x int) int {
	if u.parent[x] != x {
		u.parent[x] = u.find(u.parent[x])
	}
	return u.parent[x]
}

func (u *UnionFind) connect(x, y int) bool {
	return u.find(x) == u.find(y)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 3
		edges := [][]int{{0, 1}, {1, 2}, {2, 0}}
		source := 0
		destination := 2
		assert(validPath(n, edges, source, destination))
	}

	{
		n := 6
		edges := [][]int{{0, 1}, {0, 2}, {3, 5}, {5, 4}, {4, 3}}
		source := 0
		destination := 5
		assert(!validPath(n, edges, source, destination))
	}
}

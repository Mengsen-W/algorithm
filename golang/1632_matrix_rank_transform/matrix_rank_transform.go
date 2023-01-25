/*
 * @Date: 2023-01-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-25
 * @FilePath: /algorithm/golang/1632_matrix_rank_transform/matrix_rank_transform.go
 */

package main

import (
	"reflect"
	"sort"
)

type unionFind struct {
	p, size []int
}

func (uf *unionFind) find(x int) int {
	if uf.p[x] != x {
		uf.p[x] = uf.find(uf.p[x])
	}
	return uf.p[x]
}

func (uf *unionFind) union(a, b int) {
	pa, pb := uf.find(a), uf.find(b)
	if pa != pb {
		if uf.size[pa] > uf.size[pb] {
			uf.p[pb] = pa
			uf.size[pa] += uf.size[pb]
		} else {
			uf.p[pa] = pb
			uf.size[pb] += uf.size[pa]
		}
	}
}

func matrixRankTransform(matrix [][]int) [][]int {
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	newUnionFind := func(n int) *unionFind {
		p := make([]int, n)
		size := make([]int, n)
		for i := range p {
			p[i] = i
			size[i] = 1
		}
		return &unionFind{p, size}
	}

	m, n := len(matrix), len(matrix[0])
	type pair struct{ i, j int }
	d := map[int][]pair{}
	for i, row := range matrix {
		for j, v := range row {
			d[v] = append(d[v], pair{i, j})
		}
	}
	rowMax := make([]int, m)
	colMax := make([]int, n)
	ans := make([][]int, m)
	for i := range ans {
		ans[i] = make([]int, n)
	}
	vs := []int{}
	for v := range d {
		vs = append(vs, v)
	}
	sort.Ints(vs)
	for _, v := range vs {
		ps := d[v]
		uf := newUnionFind(m + n)
		rank := make([]int, m+n)
		for _, p := range ps {
			uf.union(p.i, p.j+m)
		}
		for _, p := range ps {
			i, j := p.i, p.j
			rank[uf.find(i)] = max(rank[uf.find(i)], max(rowMax[i], colMax[j]))
		}
		for _, p := range ps {
			i, j := p.i, p.j
			ans[i][j] = 1 + rank[uf.find(i)]
			rowMax[i], colMax[j] = ans[i][j], ans[i][j]
		}
	}
	return ans
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		matrix := [][]int{{1, 2}, {3, 4}}
		ans := [][]int{{1, 2}, {2, 3}}
		assert(matrixRankTransform(matrix), ans)
	}

	{
		matrix := [][]int{{7, 7}, {7, 7}}
		ans := [][]int{{1, 1}, {1, 1}}
		assert(matrixRankTransform(matrix), ans)
	}

	{
		matrix := [][]int{{20, -21, 14}, {-19, 4, 19}, {22, -47, 24}, {-19, 4, 19}}
		ans := [][]int{{4, 2, 3}, {1, 3, 4}, {5, 1, 6}, {1, 3, 4}}
		assert(matrixRankTransform(matrix), ans)
	}

	{
		matrix := [][]int{{7, 3, 6}, {1, 4, 5}, {9, 8, 2}}
		ans := [][]int{{5, 1, 4}, {1, 2, 3}, {6, 3, 1}}
		assert(matrixRankTransform(matrix), ans)
	}
}

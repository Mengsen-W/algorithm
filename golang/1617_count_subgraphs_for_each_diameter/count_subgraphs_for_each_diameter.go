/*
 * @Date: 2023-03-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-12
 * @FilePath: /algorithm/golang/1617_count_subgraphs_for_each_diameter/count_subgraphs_for_each_diameter.go
 */

// Package main ...
package main

import (
	"math/bits"
	"reflect"
)

func countSubgraphsForEachDiameter(n int, edges [][]int) []int {
	g := make([][]int, n)
	for _, e := range edges {
		u, v := e[0]-1, e[1]-1
		g[u] = append(g[u], v)
		g[v] = append(g[v], u)
	}
	ans := make([]int, n-1)
	var msk, nxt int
	bfs := func(u int) int {
		d := -1
		q := []int{u}
		msk ^= 1 << u
		for len(q) > 0 {
			d++
			for k := len(q); k > 0; k-- {
				u = q[0]
				q = q[1:]
				nxt = u
				for _, v := range g[u] {
					if msk>>v&1 == 1 {
						msk ^= 1 << v
						q = append(q, v)
					}
				}
			}
		}
		return d
	}
	for mask := 1; mask < 1<<n; mask++ {
		if mask&(mask-1) == 0 {
			continue
		}
		msk = mask
		cur := bits.Len(uint(msk)) - 1
		bfs(cur)
		if msk == 0 {
			msk = mask
			mx := bfs(nxt)
			ans[mx-1]++
		}
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		n := 4
		edges := [][]int{{1, 2}, {2, 3}, {2, 4}}
		ans := []int{3, 4, 0}
		assert(countSubgraphsForEachDiameter(n, edges), ans)
	}

	{
		n := 2
		edges := [][]int{{1, 2}}
		ans := []int{1}
		assert(countSubgraphsForEachDiameter(n, edges), ans)
	}

	{
		n := 3
		edges := [][]int{{1, 2}, {2, 3}}
		ans := []int{2, 1}
		assert(countSubgraphsForEachDiameter(n, edges), ans)
	}
}

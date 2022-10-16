/*
 * @Date: 2022-10-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-16
 * @FilePath: /algorithm/886_possible_bipartition/possible_bipartition.go
 */

package main

func possibleBipartition(n int, dislikes [][]int) bool {
	g := make([][]int, n)
	for _, d := range dislikes {
		x, y := d[0]-1, d[1]-1
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}
	color := make([]int, n) // color[x] = 0 表示未访问节点 x
	var dfs func(int, int) bool
	dfs = func(x, c int) bool {
		color[x] = c
		for _, y := range g[x] {
			if color[y] == c || color[y] == 0 && !dfs(y, 3^c) {
				return false
			}
		}
		return true
	}
	for i, c := range color {
		if c == 0 && !dfs(i, 1) {
			return false
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		n := 4
		dislikes := [][]int{{1, 2}, {1, 3}, {2, 4}}
		assert(possibleBipartition(n, dislikes))
	}

	{
		n := 3
		dislikes := [][]int{{1, 2}, {1, 3}, {2, 3}}
		assert(!possibleBipartition(n, dislikes))
	}

	{
		n := 5
		dislikes := [][]int{{1, 2}, {2, 3}, {3, 4}, {4, 5}, {1, 5}}
		assert(!possibleBipartition(n, dislikes))
	}
}

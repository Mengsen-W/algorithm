/*
 * @Date: 2022-03-20 02:00:18
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-20 02:10:34
 * @FilePath: /algorithm/2093_network_becomes_idle/network_becomes_idle.go
 */

package main

func networkBecomesIdle(edges [][]int, patience []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	n := len(patience)
	g := make([][]int, n)
	for _, e := range edges {
		x, y := e[0], e[1]
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}

	vis := make([]bool, n)
	vis[0] = true
	q := []int{0}
	for dist := 1; q != nil; dist++ {
		tmp := q
		q = nil
		for _, x := range tmp {
			for _, v := range g[x] {
				if vis[v] {
					continue
				}
				vis[v] = true
				q = append(q, v)
				ans = max(ans, (dist*2-1)/patience[v]*patience[v]+dist*2+1)
			}
		}
	}
	return
}

func main() {
	assert := func(a, b int) {
		if a != b {
			panic("Not Passed")
		}
	}

	assert(networkBecomesIdle([][]int{{0, 1}, {1, 2}}, []int{0, 2, 1}), 8)
	assert(networkBecomesIdle([][]int{{0, 1}, {0, 2}, {1, 2}}, []int{0, 10, 10}), 3)
}

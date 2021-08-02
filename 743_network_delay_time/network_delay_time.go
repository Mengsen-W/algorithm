/*
 * @Date: 2021-08-02 09:53:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-02 15:53:25
 */

package main

import (
	"math"
)

func networkDelayTime_Dijkstra(times [][]int, n int, k int) int {
	arcs := make([][]int, n+1)
	for i := 0; i < len(arcs); i++ {
		arcs[i] = make([]int, n+1)
		for j := 0; j < len(arcs[i]); j++ {
			arcs[i][j] = 10010
		}
	}
	for i := 1; i <= n; i++ {
		arcs[i][i] = 0
	}

	for _, time := range times {
		arcs[time[0]][time[1]] = time[2]
	}

	dist := arcs[k]
	flag := make([]bool, n+1)
	for i := 0; i < len(flag); i++ {
		flag[i] = false
	}
	flag[k] = true

	for i := 1; i <= n; i++ {
		minn := 10010
		pos := 0
		for j := 1; j <= n; j++ {
			if !flag[j] && dist[j] < minn {
				pos = j
				minn = dist[j]
			}
		}
		flag[pos] = true
		for j := 1; j <= n; j++ {
			if !flag[j] && dist[pos]+arcs[pos][j] < dist[j] {
				dist[j] = dist[pos] + arcs[pos][j]
			}
		}
	}
	ret := math.MinInt32
	for i := 1; i < len(arcs[k]); i++ {
		if arcs[k][i] > ret {
			ret = arcs[k][i]
		}
	}
	if ret == 10010 {
		return -1
	} else {
		return ret
	}
}

func networkDelayTime_floyd(times [][]int, n int, k int) int {
	arcs := make([][]int, n+1)
	for i := 0; i < len(arcs); i++ {
		arcs[i] = make([]int, n+1)
		for j := 0; j < len(arcs[i]); j++ {
			arcs[i][j] = 10010
		}
	}
	for i := 1; i <= n; i++ {
		arcs[i][i] = 0
	}

	for _, time := range times {
		arcs[time[0]][time[1]] = time[2]
	}

	for r := 1; r <= n; r++ {
		for i := 1; i <= n; i++ {
			for j := 1; j <= n; j++ {
				if arcs[i][j] > arcs[i][r]+arcs[r][j] {
					arcs[i][j] = arcs[i][r] + arcs[r][j]
				}
			}
		}
	}
	ret := math.MinInt32
	for i := 1; i < len(arcs[k]); i++ {
		if arcs[k][i] > ret {
			ret = arcs[k][i]
		}
	}
	if ret == 10010 {
		return -1
	} else {
		return ret
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		times := [][]int{{2, 1, 1}, {2, 3, 1}, {3, 4, 1}}
		n := 4
		k := 2
		ans := 2
		assert(networkDelayTime_Dijkstra(times, n, k) == ans)
		assert(networkDelayTime_floyd(times, n, k) == ans)
	}
	{
		times := [][]int{{1, 2, 1}}
		n := 2
		k := 1
		ans := 1
		assert(networkDelayTime_Dijkstra(times, n, k) == ans)
		assert(networkDelayTime_floyd(times, n, k) == ans)
	}
	{
		times := [][]int{{1, 2, 1}}
		n := 2
		k := 2
		ans := -1
		assert(networkDelayTime_Dijkstra(times, n, k) == ans)
		assert(networkDelayTime_floyd(times, n, k) == ans)
	}
}

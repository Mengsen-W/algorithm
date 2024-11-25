// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
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
	}
	return ret
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
	}
	return ret
}

func main() {
	tests := []struct {
		times [][]int
		n     int
		k     int
		ans   int
	}{
		{[][]int{{2, 1, 1}, {2, 3, 1}, {3, 4, 1}}, 4, 2, 2},
		{[][]int{{1, 2, 1}}, 2, 1, 1},
		{[][]int{{1, 2, 1}}, 2, 2, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, networkDelayTime_Dijkstra(test.times, test.n, test.k), index)
		assert.Equal(&testing.T{}, test.ans, networkDelayTime_floyd(test.times, test.n, test.k), index)
	}
}

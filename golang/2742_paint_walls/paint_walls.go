// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func paintWalls(cost []int, time []int) int {
	n := len(cost)
	f := make([]int, 2*n+1)
	for k := range f {
		f[k] = math.MaxInt / 2
	}
	f[n] = 0
	for i := 0; i < n; i++ {
		g := make([]int, n*2+1)
		for k := range g {
			g[k] = math.MaxInt / 2
		}
		for j := 0; j <= n*2; j++ {
			// 付费
			g[min(j+time[i], n*2)] = min(g[min(j+time[i], n*2)], f[j]+cost[i])
			// 免费
			if j > 0 {
				g[j-1] = min(g[j-1], f[j])
			}
		}
		f = g
	}
	res := math.MaxInt
	for i := n; i < len(f); i++ {
		res = min(res, f[i])
	}
	return res
}

func main() {
	tests := []struct {
		cost []int
		time []int
		ans  int
	}{
		{[]int{1, 2, 3, 2}, []int{1, 2, 3, 2}, 3},
		{[]int{2, 3, 4, 2}, []int{1, 1, 1, 1}, 4},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, paintWalls(test.cost, test.time), index)
	}
}

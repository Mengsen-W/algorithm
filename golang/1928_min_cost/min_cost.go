// Package main ...
package main

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func minCost(maxTime int, edges [][]int, passingFees []int) int {
	n := len(passingFees)
	f := make([][]int, maxTime+1)
	for i := range f {
		f[i] = make([]int, n)
		for j := range f[i] {
			f[i][j] = math.MaxInt32
		}
	}
	f[0][0] = passingFees[0]
	for t := 1; t <= maxTime; t++ {
		for _, edge := range edges {
			i, j, cost := edge[0], edge[1], edge[2]
			if cost <= t {
				if f[t-cost][j] != math.MaxInt32 {
					f[t][i] = min(f[t][i], f[t-cost][j]+passingFees[i])
				}
				if f[t-cost][i] != math.MaxInt32 {
					f[t][j] = min(f[t][j], f[t-cost][i]+passingFees[j])
				}
			}
		}
	}

	ans := math.MaxInt32
	for t := 1; t <= maxTime; t++ {
		ans = min(ans, f[t][n-1])
	}
	if ans == math.MaxInt32 {
		return -1
	}
	return ans
}

func main() {
	tests := []struct {
		maxTime     int
		edges       [][]int
		passingFees []int
		ans         int
	}{
		{30, [][]int{{0, 1, 10}, {1, 2, 10}, {2, 5, 10}, {0, 3, 1}, {3, 4, 10}, {4, 5, 15}}, []int{5, 1, 2, 20, 20, 3}, 11},
		{29, [][]int{{0, 1, 10}, {1, 2, 10}, {2, 5, 10}, {0, 3, 1}, {3, 4, 10}, {4, 5, 15}}, []int{5, 1, 2, 20, 20, 3}, 48},
		{25, [][]int{{0, 1, 10}, {1, 2, 10}, {2, 5, 10}, {0, 3, 1}, {3, 4, 10}, {4, 5, 15}}, []int{5, 1, 2, 20, 20, 3}, -1},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.ans, minCost(test.maxTime, test.edges, test.passingFees), index)
	}
}

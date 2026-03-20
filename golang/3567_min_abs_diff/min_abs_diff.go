// Package main ...
package main

import (
	"fmt"
	"math"
	"reflect"
	"sort"
)

func minAbsDiff(grid [][]int, k int) [][]int {
	m, n := len(grid), len(grid[0])
	res := make([][]int, m-k+1)
	for i := range res {
		res[i] = make([]int, n-k+1)
	}
	for i := 0; i+k <= m; i++ {
		for j := 0; j+k <= n; j++ {
			kgrid := []int{}
			for x := i; x < i+k; x++ {
				for y := j; y < j+k; y++ {
					kgrid = append(kgrid, grid[x][y])
				}
			}
			kmin := math.MaxInt
			sort.Ints(kgrid)
			for t := 1; t < len(kgrid); t++ {
				if kgrid[t] == kgrid[t-1] {
					continue
				}
				kmin = min(kmin, kgrid[t]-kgrid[t-1])
			}
			if kmin != math.MaxInt {
				res[i][j] = kmin
			}
		}
	}
	return res
}

func main() {
	tests := []struct {
		grid [][]int
		k    int
		want [][]int
	}{
		{[][]int{{1, 8}, {3, -2}}, 2, [][]int{{2}}},
		{[][]int{{3, -1}}, 1, [][]int{{0, 0}}},
		{[][]int{{1, -2, 3}, {2, 3, 5}}, 2, [][]int{{1, 2}}},
	}

	for _, tt := range tests {
		got := minAbsDiff(tt.grid, tt.k)
		if !reflect.DeepEqual(got, tt.want) {
			fmt.Printf("minAbsDiff(%v, %v) = %v, want %v\n", tt.grid, tt.k, got, tt.want)
		}
	}
}

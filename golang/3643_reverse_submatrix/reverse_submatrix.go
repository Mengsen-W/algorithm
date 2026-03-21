// Package main ...
package main

import "reflect"

func reverseSubmatrix(grid [][]int, x int, y int, k int) [][]int {
	for i0, i1 := x, x+k-1; i0 < i1; i0, i1 = i0+1, i1-1 {
		for j := y; j < y+k; j++ {
			grid[i0][j], grid[i1][j] = grid[i1][j], grid[i0][j]
		}
	}
	return grid
}

func main() {
	tests := []struct {
		grid   [][]int
		x      int
		y      int
		k      int
		result [][]int
	}{
		{
			[][]int{{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}, {13, 14, 15, 16}},
			1,
			0,
			3,
			[][]int{{1, 2, 3, 4}, {13, 14, 15, 8}, {9, 10, 11, 12}, {5, 6, 7, 16}},
		},
		{
			[][]int{{3, 4, 2, 3}, {2, 3, 4, 2}},
			0,
			2,
			2,
			[][]int{{3, 4, 4, 2}, {2, 3, 2, 3}},
		},
	}

	for _, test := range tests {
		result := reverseSubmatrix(test.grid, test.x, test.y, test.k)
		if !reflect.DeepEqual(result, test.result) {
			panic("error")
		}
	}
}

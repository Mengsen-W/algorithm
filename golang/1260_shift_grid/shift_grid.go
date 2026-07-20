// Package main ...
package main

import (
	"fmt"
	"reflect"
)

func shiftGrid(grid [][]int, k int) [][]int {
	m, n := len(grid), len(grid[0])
	ans := make([][]int, m)
	for i := range ans {
		ans[i] = make([]int, n)
	}
	for i, row := range grid {
		for j, v := range row {
			index1 := (i*n + j + k) % (m * n)
			ans[index1/n][index1%n] = v
		}
	}
	return ans
}

func main() {
	tests := []struct {
		grid [][]int
		k    int
		ans  [][]int
	}{
		{
			[][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
			1,
			[][]int{{9, 1, 2}, {3, 4, 5}, {6, 7, 8}},
		},
		{
			[][]int{{3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}, {12, 0, 21, 13}},
			4,
			[][]int{{12, 0, 21, 13}, {3, 8, 1, 9}, {19, 7, 2, 5}, {4, 6, 11, 10}},
		},
		{
			[][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
			9,
			[][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}},
		},
	}

	for _, test := range tests {
		ans := shiftGrid(test.grid, test.k)
		if !reflect.DeepEqual(ans, test.ans) {
			fmt.Println("test failed", test.grid, test.k, test.ans, ans)
		}
	}
}

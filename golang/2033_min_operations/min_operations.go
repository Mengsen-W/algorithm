// Package main ...
package main

import "sort"

func minOperations(grid [][]int, x int) int {
	nums := []int{}
	m, n := len(grid), len(grid[0])
	base := grid[0][0]

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if (grid[i][j]-base)%x != 0 {
				return -1
			}
			nums = append(nums, grid[i][j])
		}
	}

	sort.Ints(nums)
	choose := nums[len(nums)/2]
	ans := 0
	for _, num := range nums {
		diff := num - choose
		if diff < 0 {
			diff = -diff
		}
		ans += diff / x
	}
	return ans
}

func main() {
	tests := []struct {
		grid [][]int
		x    int
		ans  int
	}{
		{[][]int{{2, 4}, {6, 8}}, 2, 4},
		{[][]int{{1, 5}, {2, 3}}, 1, 5},
		{[][]int{{1, 2}, {3, 4}}, 2, -1},
	}

	for _, test := range tests {
		ans := minOperations(test.grid, test.x)
		if ans != test.ans {
			panic("error")
		}
	}
}

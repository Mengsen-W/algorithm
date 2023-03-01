/*
 * @Date: 2023-03-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-01
 * @FilePath: /algorithm/golang/2373_largest_local/largest_local.go
 */

package main

import "reflect"

func largestLocal(grid [][]int) [][]int {
	n := len(grid)
	ans := make([][]int, n-2)
	for i := 1; i < n-1; i++ {
		row := make([]int, n-2)
		for j := 1; j < n-1; j++ {
			mx := grid[i][j]
			for r := i - 1; r <= i+1; r++ {
				for c := j - 1; c <= j+1; c++ {
					if grid[r][c] > mx {
						mx = grid[r][c]
					}
				}
			}
			row[j-1] = mx
		}
		ans[i-1] = row
	}
	return ans
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		grid := [][]int{{9, 9, 8, 1}, {5, 6, 2, 6}, {8, 2, 6, 4}, {6, 2, 2, 2}}
		ans := [][]int{{9, 9}, {8, 6}}
		assert(largestLocal(grid), ans)
	}

	{
		grid := [][]int{{1, 1, 1, 1, 1}, {1, 1, 1, 1, 1}, {1, 1, 2, 1, 1}, {1, 1, 1, 1, 1}, {1, 1, 1, 1, 1}}
		ans := [][]int{{2, 2, 2}, {2, 2, 2}, {2, 2, 2}}
		assert(largestLocal(grid), ans)
	}
}

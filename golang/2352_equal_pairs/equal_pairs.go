/*
 * @Date: 2023-06-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-06
 * @FilePath: /algorithm/golang/2352_equal_pairs/equal_pairs.go
 */

// Package main ...
package main

import "fmt"

func equalPairs(grid [][]int) int {
	n := len(grid)
	cnt := make(map[string]int)
	for _, row := range grid {
		cnt[fmt.Sprint(row)]++
	}
	res := 0
	for j := 0; j < n; j++ {
		var arr []int
		for i := 0; i < n; i++ {
			arr = append(arr, grid[i][j])
		}
		if val, ok := cnt[fmt.Sprint(arr)]; ok {
			res += val
		}
	}

	return res
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		grid := [][]int{{3, 2, 1}, {1, 7, 6}, {2, 7, 7}}
		ans := 1
		assert(equalPairs(grid) == ans)
	}

	{
		grid := [][]int{{3, 1, 2, 2}, {1, 4, 4, 5}, {2, 4, 2, 2}, {2, 4, 2, 2}}
		ans := 3
		assert(equalPairs(grid) == ans)
	}
}

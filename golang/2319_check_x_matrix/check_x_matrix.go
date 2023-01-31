/*
 * @Date: 2023-01-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-31
 * @FilePath: /algorithm/golang/2319_check_x_matrix/check_x_matrix.go
 */

package main

func checkXMatrix(grid [][]int) bool {
	for i, row := range grid {
		for j, x := range row {
			if i == j || i+j == len(grid)-1 {
				if x == 0 {
					return false
				}
			} else if x != 0 {
				return false
			}
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		grid := [][]int{{2, 0, 0, 1}, {0, 3, 1, 0}, {0, 5, 2, 0}, {4, 0, 0, 2}}
		ans := true
		assert(checkXMatrix(grid) == ans)
	}

	{
		grid := [][]int{{5, 7, 0}, {0, 3, 1}, {0, 5, 0}}
		ans := false
		assert(checkXMatrix(grid) == ans)
	}
}

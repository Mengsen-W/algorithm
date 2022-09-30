/*
 * @Date: 2022-09-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-30
 * @FilePath: /algorithm/01.08_set_zeroes/set_zeroes.go
 */

package main

import "reflect"

func setZeroes(matrix [][]int) {
	n, m := len(matrix), len(matrix[0])
	col0 := false
	for _, r := range matrix {
		if r[0] == 0 {
			col0 = true
		}
		for j := 1; j < m; j++ {
			if r[j] == 0 {
				r[0] = 0
				matrix[0][j] = 0
			}
		}
	}
	for i := n - 1; i >= 0; i-- {
		for j := 1; j < m; j++ {
			if matrix[i][0] == 0 || matrix[0][j] == 0 {
				matrix[i][j] = 0
			}
		}
		if col0 {
			matrix[i][0] = 0
		}
	}
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		matrix := [][]int{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}
		ans := [][]int{{1, 0, 1}, {0, 0, 0}, {1, 0, 1}}
		setZeroes(matrix)
		assert(matrix, ans)
	}

	{
		matrix := [][]int{{0, 1, 2, 0}, {3, 4, 5, 2}, {1, 3, 1, 5}}
		ans := [][]int{{0, 0, 0, 0}, {0, 4, 5, 0}, {0, 3, 1, 0}}
		setZeroes(matrix)
		assert(matrix, ans)
	}
}

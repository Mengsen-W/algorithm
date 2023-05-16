/*
 * @Date: 2023-05-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-16
 * @FilePath: /algorithm/golang/1072_max_equal_rows_after_flips/max_equal_rows_after_flips.go
 */

// Package main ...
package main

func maxEqualRowsAfterFlips(matrix [][]int) int {
	m, n := len(matrix), len(matrix[0])
	mp := make(map[string]int)
	for i := 0; i < m; i++ {
		arr := make([]byte, n)
		for j := 0; j < n; j++ {
			// 如果 matrix[i][0] 为 1，则对该行元素进行翻转
			if matrix[i][j]^matrix[i][0] == 0 {
				arr[j] = '0'
			} else {
				arr[j] = '1'
			}
		}
		s := string(arr)
		mp[s]++
	}
	res := 0
	for _, value := range mp {
		if value > res {
			res = value
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
		matrix := [][]int{{0, 1}, {1, 1}}
		ans := 1
		assert(maxEqualRowsAfterFlips(matrix) == ans)
	}

	{
		matrix := [][]int{{0, 1}, {1, 0}}
		ans := 2
		assert(maxEqualRowsAfterFlips(matrix) == ans)
	}

	{
		matrix := [][]int{{0, 0, 0}, {0, 0, 1}, {1, 1, 0}}
		ans := 2
		assert(maxEqualRowsAfterFlips(matrix) == ans)
	}
}

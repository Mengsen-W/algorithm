/*
 * @Date: 2022-02-15 02:05:16
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-15 02:24:12
 * @FilePath: /algorithm/1380_lucky_numbers/lucky_numbers.go
 */

package main

import "reflect"

func luckyNumbers(matrix [][]int) (ans []int) {
	minRow := make([]int, len(matrix))
	maxCol := make([]int, len(matrix[0]))
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	for i, row := range matrix {
		minRow[i] = row[0]
		for j, x := range row {
			minRow[i] = min(minRow[i], x)
			maxCol[j] = max(maxCol[j], x)
		}
	}
	for i, row := range matrix {
		for j, x := range row {
			if x == minRow[i] && x == maxCol[j] {
				ans = append(ans, x)
			}
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(luckyNumbers([][]int{{3, 7, 8}, {9, 11, 13}, {15, 16, 17}}),
		([]int{15}))
	assert(luckyNumbers([][]int{{1, 10, 4, 2}, {9, 3, 8, 7}, {15, 16, 17, 12}}), ([]int{12}))
	assert(luckyNumbers([][]int{{7, 8}, {1, 2}}), ([]int{7}))
}

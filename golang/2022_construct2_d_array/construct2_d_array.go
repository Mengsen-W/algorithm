/*
 * @Date: 2022-01-01 02:00:46
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-01 02:26:38
 */

package main

import "reflect"

func construct2DArray(original []int, m, n int) [][]int {
	k := len(original)
	if k != m*n {
		return nil
	}
	ans := make([][]int, 0, m)
	for i := 0; i < k; i += n {
		ans = append(ans, original[i:i+n])
	}
	return ans
}

func main() {
	assert := func(a, b [][]int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(construct2DArray([]int{1, 2, 3, 4, 5, 6}, 2, 3), [][]int{{1, 2, 3}, {4, 5, 6}})
	assert(construct2DArray([]int{1, 2, 3}, 1, 3), [][]int{{1, 2, 3}})
	assert(construct2DArray([]int{1, 2}, 1, 1), nil)
	assert(construct2DArray([]int{3}, 1, 2), nil)
}

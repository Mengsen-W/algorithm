/*
 * @Date: 2022-06-08 09:41:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-08 09:47:32
 * @FilePath: /algorithm/1037_is_boomerang/is_boomerang.go
 */

package main

func isBoomerang(points [][]int) bool {
	v1 := [2]int{points[1][0] - points[0][0], points[1][1] - points[0][1]}
	v2 := [2]int{points[2][0] - points[0][0], points[2][1] - points[0][1]}
	return v1[0]*v2[1]-v1[1]*v2[0] != 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(isBoomerang([][]int{{1, 1}, {2, 3}, {3, 2}}) == true)
	assert(isBoomerang([][]int{{1, 1}, {2, 2}, {3, 3}}) == false)
}

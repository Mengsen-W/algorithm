/*
 * @Date: 2022-05-26 10:19:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-26 11:03:41
 * @FilePath: /algorithm/699_falling_squares/falling_squares.go
 */

package main

import "reflect"

func fallingSquares(positions [][]int) []int {
	n := len(positions)
	heights := make([]int, n)
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	for i, p := range positions {
		left1, right1 := p[0], p[0]+p[1]-1
		heights[i] = p[1]
		for j, q := range positions[:i] {
			left2, right2 := q[0], q[0]+q[1]-1
			if right1 >= left2 && right2 >= left1 {
				heights[i] = max(heights[i], heights[j]+p[1])
			}
		}
	}
	for i := 1; i < n; i++ {
		heights[i] = max(heights[i], heights[i-1])
	}
	return heights
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(fallingSquares([][]int{{1, 2}, {2, 3}, {6, 1}}), []int{2, 5, 5})
	assert(fallingSquares([][]int{{100, 100}, {200, 100}}), []int{100, 100})
}

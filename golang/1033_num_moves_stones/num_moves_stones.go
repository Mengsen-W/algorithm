/*
 * @Date: 2023-04-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-30
 * @FilePath: /algorithm/golang/1033_num_moves_stones/num_moves_stones.go
 */

// Package main ...
package main

import "reflect"

func numMovesStones(a int, b int, c int) []int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}

	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	x := min(min(a, b), c)
	z := max(max(a, b), c)
	y := a + b + c - x - z
	res := []int{2, z - x - 2}
	if (z-y) == 1 && (y-x) == 1 {
		res[0] = 0
	} else if (z-y) <= 2 || (y-x) <= 2 {
		res[0] = 1
	}
	return res
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		a := 1
		b := 2
		c := 5
		ans := []int{1, 2}
		assert(numMovesStones(a, b, c), ans)
	}

	{
		a := 4
		b := 3
		c := 2
		ans := []int{0, 0}
		assert(numMovesStones(a, b, c), ans)
	}

}

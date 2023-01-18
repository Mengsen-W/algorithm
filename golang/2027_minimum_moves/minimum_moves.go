/*
 * @Date: 2022-12-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-27
 * @FilePath: /algorithm/2027_minimum_moves/minimum_moves.go
 */

package main

func minimumMoves(s string) (res int) {
	covered := -1
	for i, c := range s {
		if c == 'X' && i > covered {
			res++
			covered = i + 2
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		s := "XXX"
		ans := 1
		assert(minimumMoves(s) == ans)
	}

	{
		s := "XXOX"
		ans := 2
		assert(minimumMoves(s) == ans)
	}

	{
		s := "OOOO"
		ans := 0
		assert(minimumMoves(s) == ans)
	}
}

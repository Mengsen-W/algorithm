/*
 * @Date: 2022-12-21
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-21
 * @FilePath: /algorithm/1753_maximum_score/maximum_score.go
 */

package main

func maximumScore(a, b, c int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	sum := a + b + c
	maxVal := max(max(a, b), c)
	if sum < maxVal*2 {
		return sum - maxVal
	} else {
		return sum / 2
	}
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		a := 2
		b := 4
		c := 6
		ans := 6
		assert(maximumScore(a, b, c) == ans)
	}

	{
		a := 4
		b := 4
		c := 6
		ans := 7
		assert(maximumScore(a, b, c) == ans)
	}

	{
		a := 1
		b := 8
		c := 8
		ans := 8
		assert(maximumScore(a, b, c) == ans)
	}
}

/*
 * @Date: 2022-12-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-25
 * @FilePath: /algorithm/1739_minimum_boxes/minimum_boxes.go
 */

package main

import "math"

func minimumBoxes(n int) int {
	g := func(x int) int { return x * (x + 1) * (x + 2) / 6 }
	i := int(math.Pow(6*float64(n), 1.0/3))
	if g(i) < n {
		i += 1
	}
	n -= g(i - 1)
	j := int(math.Ceil(1.0 * (math.Sqrt(8*float64(n)+1) - 1) / 2))
	return (i-1)*i/2 + j
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 3
		ans := 3
		assert(minimumBoxes(n) == ans)
	}

	{
		n := 4
		ans := 3
		assert(minimumBoxes(n) == ans)
	}

	{
		n := 10
		ans := 6
		assert(minimumBoxes(n) == ans)
	}

}

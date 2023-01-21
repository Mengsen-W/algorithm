/*
 * @Date: 2023-01-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-21
 * @FilePath: /algorithm/golang/1824_min_side_jumps/min_side_jumps.go
 */

package main

import "math"

func minSideJumps(obstacles []int) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	d := [3]int{1, 0, 1}
	for _, x := range obstacles[1:] {
		minCnt := math.MaxInt / 2
		for j := 0; j < 3; j++ {
			if j == x-1 {
				d[j] = math.MaxInt / 2
			} else {
				minCnt = min(minCnt, d[j])
			}
		}
		for j := 0; j < 3; j++ {
			if j != x-1 {
				d[j] = min(d[j], minCnt+1)
			}
		}
	}
	return min(min(d[0], d[1]), d[2])
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		obstacles := []int{0, 1, 2, 3, 0}
		ans := 2
		assert(minSideJumps(obstacles) == ans)
	}

	{
		obstacles := []int{0, 1, 1, 3, 3, 0}
		ans := 0
		assert(minSideJumps(obstacles) == ans)
	}

	{
		obstacles := []int{0, 2, 1, 0, 3, 0}
		ans := 2
		assert(minSideJumps(obstacles) == ans)
	}
}

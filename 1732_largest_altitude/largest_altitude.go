/*
 * @Date: 2022-11-19
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-19
 * @FilePath: /algorithm/1732_largest_altitude/largest_altitude.go
 */

package main

func largestAltitude(gain []int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	total := 0
	for _, x := range gain {
		total += x
		ans = max(ans, total)
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
		gain := []int{-5, 1, 5, 0, -7}
		ans := 1
		assert(largestAltitude(gain) == ans)
	}

	{
		gain := []int{-4, -3, -2, -1, 4, 3, 2}
		ans := 0
		assert(largestAltitude(gain) == ans)
	}
}

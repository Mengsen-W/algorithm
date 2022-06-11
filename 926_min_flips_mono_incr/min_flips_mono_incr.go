/*
 * @Date: 2022-06-11 21:45:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-11 21:47:27
 * @FilePath: /algorithm/926_min_flips_mono_incr/min_flips_mono_incr.go
 */

package main

func minFlipsMonoIncr(s string) int {
	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}
	dp0, dp1 := 0, 0
	for _, c := range s {
		dp0New, dp1New := dp0, min(dp0, dp1)
		if c == '1' {
			dp0New++
		} else {
			dp1New++
		}
		dp0, dp1 = dp0New, dp1New
	}
	return min(dp0, dp1)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(minFlipsMonoIncr("00110") == 1)
	assert(minFlipsMonoIncr("010110") == 2)
	assert(minFlipsMonoIncr("00011000") == 2)
}

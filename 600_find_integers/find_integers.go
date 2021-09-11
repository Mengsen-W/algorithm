/*
 * @Date: 2021-09-11 07:53:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-11 07:57:56
 */

package main

func findIntegers(n int) (ans int) {
	dp := [31]int{1, 1}
	for i := 2; i < 31; i++ {
		dp[i] = dp[i-1] + dp[i-2]
	}
	for i, pre := 29, 0; i >= 0; i-- {
		val := 1 << i
		if n&val > 0 {
			n ^= val
			ans += dp[i+1]
			if pre == 1 {
				break
			}
			pre = 1
		} else {
			pre = 0
		}
		if i == 0 {
			ans++
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
	assert(findIntegers(5) == 5)
}

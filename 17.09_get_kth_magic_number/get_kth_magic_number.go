/*
 * @Date: 2022-09-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-28
 * @FilePath: /algorithm/17.09_get_kth_magic_number/get_kth_magic_number.go
 */

package main

func getKthMagicNumber(k int) int {
	dp := make([]int, k+1)
	dp[1] = 1
	p3, p5, p7 := 1, 1, 1
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	for i := 2; i <= k; i++ {
		x2, x3, x5 := dp[p3]*3, dp[p5]*5, dp[p7]*7
		dp[i] = min(min(x2, x3), x5)
		if dp[i] == x2 {
			p3++
		}
		if dp[i] == x3 {
			p5++
		}
		if dp[i] == x5 {
			p7++
		}
	}
	return dp[k]
}

func main() {
	func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}(getKthMagicNumber(5) == 9)
}

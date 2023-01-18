/*
 * @Date: 2022-11-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-09
 * @FilePath: /algorithm/764_order_of_largest_plus_sign/order_of_largest_plus_sign.go
 */

package main

func orderOfLargestPlusSign(n int, mines [][]int) (ans int) {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}

	min := func(a, b int) int {
		if a > b {
			return b
		}
		return a
	}

	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
		for j := range dp[i] {
			dp[i][j] = n
		}
	}
	banned := map[int]bool{}
	for _, p := range mines {
		banned[p[0]*n+p[1]] = true
	}
	for i := 0; i < n; i++ {
		count := 0
		/* left */
		for j := 0; j < n; j++ {
			if banned[i*n+j] {
				count = 0
			} else {
				count++
			}
			dp[i][j] = min(dp[i][j], count)
		}
		count = 0
		/* right */
		for j := n - 1; j >= 0; j-- {
			if banned[i*n+j] {
				count = 0
			} else {
				count++
			}
			dp[i][j] = min(dp[i][j], count)
		}
	}
	for i := 0; i < n; i++ {
		count := 0
		/* up */
		for j := 0; j < n; j++ {
			if banned[j*n+i] {
				count = 0
			} else {
				count++
			}
			dp[j][i] = min(dp[j][i], count)
		}
		count = 0
		/* down */
		for j := n - 1; j >= 0; j-- {
			if banned[j*n+i] {
				count = 0
			} else {
				count++
			}
			dp[j][i] = min(dp[j][i], count)
			ans = max(ans, dp[j][i])
		}
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 5
		mines := [][]int{{4, 2}}
		ans := 2
		assert(orderOfLargestPlusSign(n, mines) == ans)
	}

	{
		n := 1
		mines := [][]int{{0, 0}}
		ans := 0
		assert(orderOfLargestPlusSign(n, mines) == ans)
	}
}

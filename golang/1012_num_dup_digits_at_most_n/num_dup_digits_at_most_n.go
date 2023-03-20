/*
 * @Date: 2023-03-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-20
 * @FilePath: /algorithm/golang/1012_num_dup_digits_at_most_n/num_dup_digits_at_most_n.go
 */

// Package main ...
package main

func numDupDigitsAtMostN(n int) int {
	f := func(n int) int {
		nums := []int{}
		for ; n > 0; n /= 10 {
			nums = append(nums, n%10)
		}
		dp := [11][1 << 11]int{}
		for i := range dp {
			for j := range dp[i] {
				dp[i][j] = -1
			}
		}
		var dfs func(int, int, bool, bool) int
		dfs = func(pos int, mask int, lead bool, limit bool) int {
			if pos < 0 {
				if lead {
					return 0
				}
				return 1
			}
			if !lead && !limit && dp[pos][mask] != -1 {
				return dp[pos][mask]
			}
			up := 9
			if limit {
				up = nums[pos]
			}
			ans := 0
			for i := 0; i <= up; i++ {
				if mask>>i&1 == 1 {
					continue
				}
				if i == 0 && lead {
					ans += dfs(pos-1, mask, lead, limit && i == up)
				} else {
					ans += dfs(pos-1, mask|1<<i, false, limit && i == up)
				}
			}
			if !lead && !limit {
				dp[pos][mask] = ans
			}
			return ans
		}
		return dfs(len(nums)-1, 0, true, true)
	}
	return n - f(n)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		n := 20
		ans := 1
		assert(numDupDigitsAtMostN(n) == ans)
	}

	{
		n := 100
		ans := 10
		assert(numDupDigitsAtMostN(n) == ans)
	}

	{
		n := 1000
		ans := 262
		assert(numDupDigitsAtMostN(n) == ans)
	}
}

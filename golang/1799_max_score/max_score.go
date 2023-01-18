/*
 * @Date: 2022-12-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-22
 * @FilePath: /algorithm/1799_max_score/max_score.go
 */

package main

import "math/bits"

func maxScore(nums []int) int {
	var gcd func(a, b int) int
	gcd = func(a, b int) int {
		if a%b == 0 {
			return b
		}
		return gcd(b, a%b)
	}

	max := func(a, b int) int {
		if a >= b {
			return a
		} else {
			return b
		}
	}

	m := len(nums)
	dp := make([]int, 1<<m)
	gcd_tmp := make([][]int, m)
	for i := 0; i < m; i++ {
		gcd_tmp[i] = make([]int, m)
	}

	for i := 0; i < m; i++ {
		for j := i + 1; j < m; j++ {
			gcd_tmp[i][j] = gcd(nums[i], nums[j])
		}
	}

	all := 1 << m
	for s := 1; s < all; s++ {
		t := bits.OnesCount(uint(s))
		if t&1 != 0 {
			continue
		}
		for i := 0; i < m; i++ {
			if (s>>i)&1 != 0 {
				for j := i + 1; j < m; j++ {
					if (s>>j)&1 != 0 {
						dp[s] = max(dp[s], dp[s^(1<<i)^(1<<j)]+t/2*gcd_tmp[i][j])
					}
				}
			}
		}
	}
	return dp[all-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{1, 2}
		ans := 1
		assert(maxScore(nums) == ans)
	}

	{
		nums := []int{3, 4, 6, 8}
		ans := 11
		assert(maxScore(nums) == ans)
	}

	{
		nums := []int{1, 2, 3, 4, 5, 6}
		ans := 14
		assert(maxScore(nums) == ans)
	}
}

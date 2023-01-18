/*
 * @Date: 2021-09-20 09:17:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-20 09:55:19
 */

package main

func findNumberOfLIS(nums []int) (ans int) {
	maxLen := 0
	n := len(nums)
	dp := make([]int, n)
	cnt := make([]int, n)
	for i, x := range nums {
		dp[i] = 1
		cnt[i] = 1
		for j, y := range nums[:i] {
			if x > y {
				if dp[j]+1 > dp[i] {
					dp[i] = dp[j] + 1
					cnt[i] = cnt[j] // 重置计数
				} else if dp[j]+1 == dp[i] {
					cnt[i] += cnt[j]
				}
			}
		}
		if dp[i] > maxLen {
			maxLen = dp[i]
			ans = cnt[i] // 重置计数
		} else if dp[i] == maxLen {
			ans += cnt[i]
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
	{
		nums := []int{1, 3, 5, 4, 7}
		assert(findNumberOfLIS(nums) == 2)
	}
	{
		nums := []int{2, 2, 2, 2, 2}
		assert(findNumberOfLIS(nums) == 5)
	}
}

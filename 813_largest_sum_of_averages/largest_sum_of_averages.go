/*
 * @Date: 2022-11-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-28
 * @FilePath: /algorithm/813_largest_sum_of_averages/largest_sum_of_averages.go
 */

package main

import "math"

func largestSumOfAverages(nums []int, k int) float64 {
	n := len(nums)
	prefix := make([]float64, n+1)
	for i, x := range nums {
		prefix[i+1] = prefix[i] + float64(x)
	}
	dp := make([]float64, n+1)
	for i := 1; i <= n; i++ {
		dp[i] = prefix[i] / float64(i)
	}
	for j := 2; j <= k; j++ {
		for i := n; i >= j; i-- {
			for x := j - 1; x < i; x++ {
				dp[i] = math.Max(dp[i], dp[x]+(prefix[i]-prefix[x])/float64(i-x))
			}
		}
	}
	return dp[n]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{9, 1, 2, 3, 9}
		k := 3
		ans := 20.0000000000
		assert(largestSumOfAverages(nums, k) == ans)
	}

	{
		nums := []int{1, 2, 3, 4, 5, 6, 7}
		k := 4
		ans := 20.50000
		assert(largestSumOfAverages(nums, k) == ans)
	}
}

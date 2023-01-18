/*
 * @Date: 2022-09-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-20
 * @FilePath: /algorithm/698_can_partition_k_subsets/can_partition_k_subsets.go
 */

package main

import "sort"

func canPartitionKSubsets(nums []int, k int) bool {
	all := 0
	for _, v := range nums {
		all += v
	}
	if all%k > 0 {
		return false
	}
	per := all / k
	sort.Ints(nums)
	n := len(nums)
	if nums[n-1] > per {
		return false
	}

	dp := make([]bool, 1<<n)
	dp[0] = true
	curSum := make([]int, 1<<n)
	for i, v := range dp {
		if !v {
			continue
		}
		for j, num := range nums {
			if curSum[i]+num > per {
				break
			}
			if i>>j&1 == 0 {
				next := i | 1<<j
				if !dp[next] {
					curSum[next] = (curSum[i] + nums[j]) % per
					dp[next] = true

				}
			}
		}
	}
	return dp[1<<n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{4, 3, 2, 3, 5, 2, 1}
		k := 4
		assert(canPartitionKSubsets(nums, k) == true)
	}

	{
		nums := []int{1, 2, 3, 4}
		k := 3
		assert(canPartitionKSubsets(nums, k) == false)
	}
}

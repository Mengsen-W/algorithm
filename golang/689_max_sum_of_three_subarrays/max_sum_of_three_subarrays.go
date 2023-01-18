/*
 * @Date: 2021-12-09 04:47:36
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-09 05:01:50
 */

package main

import "reflect"

func maxSumOfThreeSubarrays(nums []int, k int) (ans []int) {
	var sum1, maxSum1, maxSum1Idx int
	var sum2, maxSum12, maxSum12Idx1, maxSum12Idx2 int
	var sum3, maxTotal int
	for i := k * 2; i < len(nums); i++ {
		sum1 += nums[i-k*2]
		sum2 += nums[i-k]
		sum3 += nums[i]
		if i >= k*3-1 {
			if sum1 > maxSum1 {
				maxSum1 = sum1
				maxSum1Idx = i - k*3 + 1
			}
			if maxSum1+sum2 > maxSum12 {
				maxSum12 = maxSum1 + sum2
				maxSum12Idx1, maxSum12Idx2 = maxSum1Idx, i-k*2+1
			}
			if maxSum12+sum3 > maxTotal {
				maxTotal = maxSum12 + sum3
				ans = []int{maxSum12Idx1, maxSum12Idx2, i - k + 1}
			}
			sum1 -= nums[i-k*3+1]
			sum2 -= nums[i-k*2+1]
			sum3 -= nums[i-k+1]
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(maxSumOfThreeSubarrays([]int{1, 2, 1, 2, 6, 7, 5, 1}, 2), []int{0, 3, 5})
	assert(maxSumOfThreeSubarrays([]int{1, 2, 1, 2, 1, 2, 1, 2, 1}, 2), []int{0, 2, 4})
}

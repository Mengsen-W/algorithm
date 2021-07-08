/*
 * @Date: 2021-07-08 08:50:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-08 09:00:12
 */

package main

func numSubarraysWithSum(nums []int, goal int) (ans int) {
	left1, left2 := 0, 0
	sum1, sum2 := 0, 0
	for right, num := range nums {
		sum1 += num
		for left1 <= right && sum1 > goal {
			sum1 -= nums[left1]
			left1++
		}
		sum2 += num
		for left2 <= right && sum2 >= goal {
			sum2 -= nums[left2]
			left2++
		}
		ans += left2 - left1
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed!")
		}
	}
	{
		nums := []int{1, 0, 1, 0, 1}
		assert(numSubarraysWithSum(nums, 2) == 4)
	}
	{
		nums := []int{1, 0, 1, 0, 1}
		assert(numSubarraysWithSum(nums, 2) == 4)
	}
}

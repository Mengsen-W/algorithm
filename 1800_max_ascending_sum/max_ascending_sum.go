/*
 * @Date: 2022-10-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-07
 * @FilePath: /algorithm/1800_max_ascending_sum/max_ascending_sum.go
 */

package main

func maxAscendingSum(nums []int) (ans int) {
	for i, n := 0, len(nums); i < n; {
		sum := nums[i]
		for i++; i < n && nums[i] > nums[i-1]; i++ {
			sum += nums[i]
		}
		if sum > ans {
			ans = sum
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
		nums := []int{10, 20, 30, 5, 10, 50}
		assert(maxAscendingSum(nums) == 65)
	}

	{
		nums := []int{10, 20, 30, 40, 50}
		assert(maxAscendingSum(nums) == 150)
	}

	{
		nums := []int{12, 17, 15, 13, 10, 11, 12}
		assert(maxAscendingSum(nums) == 33)
	}

	{
		nums := []int{100, 10, 1}
		assert(maxAscendingSum(nums) == 100)
	}
}

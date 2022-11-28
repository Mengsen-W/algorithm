/*
 * @Date: 2022-11-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-27
 * @FilePath: /algorithm/1752_check/check.go
 */

package main

func check(nums []int) bool {
	n := len(nums)
	x := 0
	for i := 1; i < n; i++ {
		if nums[i] < nums[i-1] {
			x = i
			break
		}
	}
	if x == 0 {
		return true
	}
	for i := x + 1; i < n; i++ {
		if nums[i] < nums[i-1] {
			return false
		}
	}
	return nums[0] >= nums[n-1]
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{3, 4, 5, 1, 2}
		assert(check(nums))
	}

	{
		nums := []int{2, 1, 3, 4}
		assert(!check(nums))
	}

	{
		nums := []int{1, 2, 3}
		assert(check(nums))
	}
}

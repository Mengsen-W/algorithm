/*
 * @Date: 2022-11-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-14
 * @FilePath: /algorithm/805_split_array_same_average/split_array_same_average.go
 */

package main

func splitArraySameAverage(nums []int) bool {
	n := len(nums)
	if n == 1 {
		return false
	}

	sum := 0
	for _, x := range nums {
		sum += x
	}
	for i := range nums {
		nums[i] = nums[i]*n - sum
	}

	m := n / 2
	left := map[int]bool{}
	for i := 1; i < 1<<m; i++ {
		tot := 0
		for j, x := range nums[:m] {
			if i>>j&1 > 0 {
				tot += x
			}
		}
		if tot == 0 {
			return true
		}
		left[tot] = true
	}

	rsum := 0
	for _, x := range nums[m:] {
		rsum += x
	}
	for i := 1; i < 1<<(n-m); i++ {
		tot := 0
		for j, x := range nums[m:] {
			if i>>j&1 > 0 {
				tot += x
			}
		}
		if tot == 0 || rsum != tot && left[-tot] {
			return true
		}
	}
	return false
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{1, 2, 3, 4, 5, 6, 7, 8}
		assert(splitArraySameAverage(nums))
	}
	{
		nums := []int{3, 1}
		assert(!splitArraySameAverage(nums))
	}
}

/*
 * @Date: 2022-11-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-24
 * @FilePath: /algorithm/795_num_subarray_bounded_max/num_subarray_bounded_max.go
 */

package main

func numSubarrayBoundedMax(nums []int, left int, right int) (res int) {
	last2, last1 := -1, -1
	for i, x := range nums {
		if left <= x && x <= right {
			last1 = i
		} else if x > right {
			last2 = i
			last1 = -1
		}
		if last1 != -1 {
			res += last1 - last2
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
		nums := []int{2, 1, 4, 3}
		left := 2
		right := 3
		ans := 3
		assert(numSubarrayBoundedMax(nums, left, right) == ans)
	}

	{
		nums := []int{2, 9, 2, 5, 6}
		left := 2
		right := 8
		ans := 7
		assert(numSubarrayBoundedMax(nums, left, right) == ans)
	}
}

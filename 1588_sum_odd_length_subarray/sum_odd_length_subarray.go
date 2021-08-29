/*
 * @Date: 2021-08-29 16:57:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-29 17:16:01
 */

package main

func sumOddLengthSubarrays(arr []int) int {
	sum := 0
	n := len(arr)
	for i := 0; i < n; i++ {
		left_count := i
		right_count := n - i - 1
		left_odd := (left_count + 1) / 2
		right_odd := (right_count + 1) / 2
		left_even := left_count/2 + 1
		right_even := right_count/2 + 1
		sum += arr[i] * (left_odd*right_odd + left_even*right_even)
	}
	return sum
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{1, 4, 2, 5, 3}
		ans := 58
		assert(sumOddLengthSubarrays(nums) == ans)
	}
	{
		nums := []int{1, 2}
		ans := 3
		assert(sumOddLengthSubarrays(nums) == ans)
	}
	{
		nums := []int{10, 11, 12}
		ans := 66
		assert(sumOddLengthSubarrays(nums) == ans)
	}
}

/*
 * @Date: 2022-04-22 09:20:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-22 09:26:04
 * @FilePath: /algorithm/396_max_rotate_function/max_rotate_function.go
 */

package main

func maxRotateFunction(nums []int) int {
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	numSum := 0
	for _, v := range nums {
		numSum += v
	}
	f := 0
	for i, num := range nums {
		f += i * num
	}
	ans := f
	for i := len(nums) - 1; i > 0; i-- {
		f += numSum - len(nums)*nums[i]
		ans = max(ans, f)
	}
	return ans
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(maxRotateFunction([]int{4, 3, 2, 6}) == 26)
	assert(maxRotateFunction([]int{100}) == 0)
}

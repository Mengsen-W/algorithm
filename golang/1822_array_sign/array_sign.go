/*
 * @Date: 2022-10-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-27
 * @FilePath: /algorithm/1822_array_sign/array_sign.go
 */

package main

func arraySign(nums []int) int {
	sign := 1
	for _, num := range nums {
		if num == 0 {
			return 0
		}
		if num < 0 {
			sign = -sign
		}
	}
	return sign
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{-1, -2, -3, -4, 3, 2, 1}
		ans := 1
		assert(arraySign(nums) == ans)
	}

	{
		nums := []int{1, 5, 0, 2, -3}
		ans := 0
		assert(arraySign(nums) == ans)
	}

	{
		nums := []int{-1, 1, -1, 1, -1}
		ans := -1
		assert(arraySign(nums) == ans)
	}
}

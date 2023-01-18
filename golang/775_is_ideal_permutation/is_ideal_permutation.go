/*
 * @Date: 2022-11-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-16
 * @FilePath: /algorithm/775_is_ideal_permutation/is_ideal_permutation.go
 */

package main

func isIdealPermutation(nums []int) bool {
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}

	for i, x := range nums {
		if abs(x-i) > 1 {
			return false
		}
	}
	return true
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{1, 0, 2}
		assert(isIdealPermutation(nums))
	}

	{
		nums := []int{1, 2, 0}
		assert(!isIdealPermutation(nums))
	}
}

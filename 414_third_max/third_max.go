/*
 * @Date: 2021-10-06 10:12:17
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-06 10:29:21
 */

package main

func thirdMax(nums []int) int {
	var a, b, c *int
	for _, num := range nums {
		num := num
		if a == nil || num > *a {
			a, b, c = &num, a, b
		} else if *a > num && (b == nil || num > *b) {
			b, c = &num, b
		} else if b != nil && *b > num && (c == nil || num > *c) {
			c = &num
		}
	}
	if c == nil {
		return *a
	}
	return *c
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{3, 2, 1}
		ans := 1
		assert(thirdMax(nums) == ans)
	}
	{
		nums := []int{2, 1}
		ans := 2
		assert(thirdMax(nums) == ans)
	}
	{
		nums := []int{2, 3, 3, 1}
		ans := 1
		assert(thirdMax(nums) == ans)
	}
}

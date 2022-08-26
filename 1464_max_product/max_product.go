/*
 * @Date: 2022-08-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-26
 * @FilePath: /algorithm/1464_max_product/max_product.go
 */

package main

func maxProduct(nums []int) int {
	a, b := nums[0], nums[1]
	if a < b {
		a, b = b, a
	}
	for _, num := range nums[2:] {
		if num > a {
			a, b = num, a
		} else if num > b {
			b = num
		}
	}
	return (a - 1) * (b - 1)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{3, 4, 5, 2}
		ans := 12
		assert(maxProduct(nums) == ans)
	}
	{
		nums := []int{1, 5, 4, 5}
		ans := 16
		assert(maxProduct(nums) == ans)
	}
	{
		nums := []int{3, 7}
		ans := 12
		assert(maxProduct(nums) == ans)
	}
}

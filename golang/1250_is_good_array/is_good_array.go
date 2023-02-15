/*
 * @Date: 2023-02-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-15
 * @FilePath: /algorithm/golang/1250_is_good_array/is_good_array.go
 */

package main

func isGoodArray(nums []int) bool {
	gcd := func(a, b int) int {
		for a != 0 {
			a, b = b%a, a
		}
		return b
	}
	g := 0
	for _, x := range nums {
		g = gcd(g, x)
		if g == 1 {
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
		nums := []int{12, 5, 7, 23}
		assert(isGoodArray(nums))
	}

	{
		nums := []int{29, 6, 10}
		assert(isGoodArray(nums))
	}

	{
		nums := []int{3, 6}
		assert(!isGoodArray(nums))
	}
}

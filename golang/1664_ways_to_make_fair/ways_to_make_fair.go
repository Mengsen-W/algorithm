/*
 * @Date: 2023-01-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-28
 * @FilePath: /algorithm/golang/1664_ways_to_make_fair/ways_to_make_fair.go
 */

package main

func waysToMakeFair(nums []int) (res int) {
	var odd1, even1, odd2, even2 int
	for i, num := range nums {
		if i&1 > 0 {
			odd2 += num
		} else {
			even2 += num
		}
	}
	for i, num := range nums {
		if i&1 > 0 {
			odd2 -= num
		} else {
			even2 -= num
		}
		if odd1+even2 == odd2+even1 {
			res++
		}
		if i&1 > 0 {
			odd1 += num
		} else {
			even1 += num
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
		nums := []int{2, 1, 6, 4}
		ans := 1
		assert(waysToMakeFair(nums) == ans)
	}

	{
		nums := []int{1, 1, 1}
		ans := 3
		assert(waysToMakeFair(nums) == ans)
	}

	{
		nums := []int{1, 2, 3}
		ans := 0
		assert(waysToMakeFair(nums) == ans)
	}
}

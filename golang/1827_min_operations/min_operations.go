/*
 * @Date: 2022-12-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-11
 * @FilePath: /algorithm/1827_min_operations/min_operations.go
 */

package main

func minOperations(nums []int) (ans int) {
	pre := nums[0] - 1
	max := func(a, b int) int {
		if b > a {
			return b
		}
		return a
	}
	for _, x := range nums {
		pre = max(pre+1, x)
		ans += pre - x
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
		nums := []int{1, 1, 1}
		ans := 3
		assert(minOperations(nums) == ans)
	}
	{
		nums := []int{1, 5, 2, 4, 1}
		ans := 14
		assert(minOperations(nums) == ans)
	}
	{
		nums := []int{8}
		ans := 0
		assert(minOperations(nums) == ans)
	}
}

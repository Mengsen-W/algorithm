/*
 * @Date: 2023-02-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-24
 * @FilePath: /algorithm/golang/2357_minimum_operations/minimum_operations.go
 */

package main

func minimumOperations(nums []int) int {
	set := map[int]struct{}{}
	for _, x := range nums {
		if x > 0 {
			set[x] = struct{}{}
		}
	}
	return len(set)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		nums := []int{1, 5, 0, 3, 5}
		ans := 3
		assert(minimumOperations(nums) == ans)
	}

	{
		nums := []int{0}
		ans := 0
		assert(minimumOperations(nums) == ans)
	}
}

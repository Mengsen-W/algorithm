/*
 * @Date: 2022-07-17
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-17
 * @FilePath: /algorithm/565_array_nesting/array_nesting.go
 */

package main

func arrayNesting(nums []int) (ans int) {
	n := len(nums)
	for i := range nums {
		cnt := 0
		for nums[i] < n {
			i, nums[i] = nums[i], n
			cnt++
		}
		if cnt > ans {
			ans = cnt
		}
	}
	return
}

func main() {
	func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}(arrayNesting([]int{5, 4, 0, 3, 1, 6, 2}) == 4)
}

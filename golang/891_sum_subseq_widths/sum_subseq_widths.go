/*
 * @Date: 2022-11-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-18
 * @FilePath: /algorithm/891_sum_subseq_widths/sum_subseq_widths.go
 */

package main

import "sort"

func sumSubseqWidths(nums []int) int {
	const mod int = 1e9 + 7
	sort.Ints(nums)
	res, x, y := 0, nums[0], 2
	for _, num := range nums[1:] {
		res = (res + num*(y-1) - x) % mod
		x = (x*2 + num) % mod
		y = y * 2 % mod
	}
	return (res + mod) % mod
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		nums := []int{2, 1, 3}
		ans := 6
		assert(sumSubseqWidths(nums) == ans)
	}

	{
		nums := []int{2}
		ans := 0
		assert(sumSubseqWidths(nums) == ans)
	}
}

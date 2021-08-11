/*
 * @Date: 2021-08-11 14:38:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-11 14:52:20
 */

package main

func numberOfArithmeticSlices(nums []int) (ans int) {
	f := make([]map[int]int, len(nums))
	for i, x := range nums {
		f[i] = map[int]int{}
		for j, y := range nums[:i] {
			d := x - y
			cnt := f[j][d]
			ans += cnt
			f[i][d] += cnt + 1
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
		nums := []int{2, 4, 6, 8, 10}
		assert(numberOfArithmeticSlices(nums) == 7)
	}

	{
		nums := []int{7, 7, 7, 7, 7}
		assert(numberOfArithmeticSlices(nums) == 16)
	}
}

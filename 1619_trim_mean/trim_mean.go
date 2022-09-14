/*
 * @Date: 2022-09-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-14
 * @FilePath: /algorithm/1619_trim_mean/trim_mean.go
 */

package main

import "sort"

func trimMean(arr []int) float64 {
	sort.Ints(arr)
	n := len(arr)
	sum := 0
	for _, x := range arr[n/20 : 19*n/20] {
		sum += x
	}
	return float64(sum*10) / float64(n*9)
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	{
		arr := []int{1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3}
		ans := 2.00000
		assert(trimMean(arr) == ans)
	}

	{
		arr := []int{6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0}
		ans := 4.00000
		assert(trimMean(arr) == ans)
	}

	{
		arr := []int{6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4,
			8, 1, 9, 5, 4, 3, 8, 5, 10, 8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4}
		ans := 4.77777777777777778
		assert(trimMean(arr) == ans)
	}

	{
		arr := []int{4, 8, 4, 10, 0, 7, 1, 3, 7, 8, 8, 3, 4, 1, 6, 2, 1, 1, 8, 0, 9, 8, 0, 3, 9, 10, 3,
			10, 1, 10, 7, 3, 2, 1, 4, 9, 10, 7, 6, 4, 0, 8, 5, 1, 2, 1, 6, 2, 5, 0, 7, 10, 9, 10,
			3, 7, 10, 5, 8, 5, 7, 6, 7, 6, 10, 9, 5, 10, 5, 5, 7, 2, 10, 7, 7, 8, 2, 0, 1, 1}
		ans := 5.291666666666666667
		assert(trimMean(arr) == ans)
	}
}

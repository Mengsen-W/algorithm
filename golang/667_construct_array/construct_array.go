/*
 * @Date: 2022-09-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-08
 * @FilePath: /algorithm/667_construct_array/construct_array.go
 */

package main

import "reflect"

func constructArray(n, k int) []int {
	ans := make([]int, 0, n)
	for i := 1; i < n-k; i++ {
		ans = append(ans, i)
	}
	for i, j := n-k, n; i <= j; i++ {
		ans = append(ans, i)
		if i != j {
			ans = append(ans, j)
		}
		j--
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		n := 3
		k := 1
		ans := []int{1, 2, 3}
		assert(constructArray(n, k), ans)
	}

	{
		n := 3
		k := 2
		ans := []int{1, 3, 2}
		assert(constructArray(n, k), ans)
	}
}

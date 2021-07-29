/*
 * @Date: 2021-07-29 09:47:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-29 11:36:52
 */

package main

import (
	"math"
	"reflect"
)

func pathInZigZagTree(label int) []int {
	reverse := func(s []int) []int {
		a := make([]int, len(s))
		copy(a, s)
		for i := len(a)/2 - 1; i >= 0; i-- {
			opp := len(a) - 1 - i
			a[i], a[opp] = a[opp], a[i]
		}
		return a
	}

	ans := []int{}
	for label >= 1 {
		ans = append(ans, label)
		label >>= 1
	}
	ans = reverse(ans)

	for i := len(ans) - 2; i > 0; i -= 2 {
		ans[i] = int(math.Pow(2, float64(i))+math.Pow(2, float64(i+1))) - 1 - ans[i]
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
		label := 14
		ans := []int{1, 3, 4, 14}
		assert(pathInZigZagTree(label), ans)
	}
	{
		label := 26
		ans := []int{1, 2, 6, 10, 26}
		assert(pathInZigZagTree(label), ans)
	}
}

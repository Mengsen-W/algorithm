/*
 * @Date: 2023-02-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-23
 * @FilePath: /algorithm/golang/1238_circular_permutation/circular_permutation.go
 */

package main

import (
	"reflect"
)

func circularPermutation(n int, start int) []int {
	ans := make([]int, 1<<n)
	for i := range ans {
		ans[i] = (i >> 1) ^ i ^ start
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
		n := 2
		start := 3
		ans := []int{3, 2, 0, 1}
		assert(circularPermutation(n, start), ans)
	}

	{
		n := 3
		start := 2
		ans := []int{2, 3, 1, 0, 4, 5, 7, 6}
		assert(circularPermutation(n, start), ans)
	}
}

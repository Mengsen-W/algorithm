/*
 * @Date: 2022-01-08 01:10:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-08 01:12:02
 */

package main

import "reflect"

func grayCode(n int) []int {
	ans := make([]int, 1<<n)
	for i := range ans {
		ans[i] = i>>1 ^ i
	}
	return ans
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	assert(grayCode(2), []int{0, 1, 3, 2})
	assert(grayCode(1), []int{0, 1})
}

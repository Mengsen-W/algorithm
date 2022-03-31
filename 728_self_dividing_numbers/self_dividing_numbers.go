/*
 * @Date: 2022-03-31 13:10:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-31 13:27:23
 * @FilePath: /algorithm/728_self_dividing_numbers/self_dividing_numbers.go
 */
package main

import (
	"reflect"
)

func selfDividingNumbers(left, right int) (ans []int) {
	isSelfDividing := func(num int) bool {
		for x := num; x > 0; x /= 10 {
			if d := x % 10; d == 0 || num%d != 0 {
				return false
			}
		}
		return true
	}
	for i := left; i <= right; i++ {
		if isSelfDividing(i) {
			ans = append(ans, i)
		}
	}
	return
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	assert(selfDividingNumbers(1, 22), []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22})
	assert(selfDividingNumbers(47, 85), []int{48, 55, 66, 77})
}

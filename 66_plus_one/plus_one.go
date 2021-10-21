/*
 * @Date: 2021-10-21 01:13:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-21 01:43:42
 */

package main

import "reflect"

func plusOne(digits []int) []int {
	for i := len(digits) - 1; i != -1; i-- {
		digits[i]++
		if digits[i]/10 == 0 {
			return digits
		}
		digits[i] = digits[i] % 10
	}
	return append([]int{1}, digits...)
}

func main() {
	assert := func(a, b []int) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}
	{
		digits := []int{1, 2, 3}
		assert(plusOne(digits), []int{1, 2, 4})
	}
	{
		digits := []int{1, 2, 9}
		assert(plusOne(digits), []int{1, 3, 0})
	}

}

/*
 * @Date: 2021-09-26 08:52:28
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-26 08:56:29
 */

package main

func getSum(a, b int) int {
	for b != 0 {
		carry := uint(a&b) << 1
		a ^= b
		b = int(carry)
	}
	return a
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(getSum(10, 2) == 12)
	assert(getSum(0, 2) == 2)
}

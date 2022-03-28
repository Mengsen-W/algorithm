/*
 * @Date: 2022-03-28 14:59:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-28 15:03:46
 * @FilePath: /algorithm/693_has_alternating_bits/has_alternating_bits.go
 */

package main

func hasAlternatingBits(n int) bool {
	a := n ^ n>>1
	return a&(a+1) == 0
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(hasAlternatingBits(5) == true)
	assert(hasAlternatingBits(7) == false)
	assert(hasAlternatingBits(11) == false)
}

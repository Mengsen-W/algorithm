/*
 * @Date: 2021-11-30 03:36:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-30 03:40:42
 */

package main

import "math"

func findNthDigit(n int) int {
	d := 1
	for count := 9; n > d*count; count *= 10 {
		n -= d * count
		d++
	}
	index := n - 1
	start := int(math.Pow10(d - 1))
	num := start + index/d
	digitIndex := index % d
	return num / int(math.Pow10(d-digitIndex-1)) % 10
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	assert(findNthDigit(3) == 3)
	assert(findNthDigit(11) == 0)
}

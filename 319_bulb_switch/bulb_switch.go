/*
 * @Date: 2021-11-15 01:29:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-15 01:35:35
 */

package main

import "math"

func bulbSwitch(n int) int {
	return int(math.Sqrt(float64(n) + 0.5))
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("assertion failed")
		}
	}
	assert(bulbSwitch(0) == 0)
	assert(bulbSwitch(1) == 1)
	assert(bulbSwitch(3) == 1)
}

/*
 * @Date: 2022-11-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-04
 * @FilePath: /algorithm/754_reach_number/reach_number.go
 */

package main

func reachNumber(target int) int {
	if target < 0 {
		target = -target
	}
	k := 0
	for target > 0 {
		k++
		target -= k
	}
	if target%2 == 0 {
		return k
	}
	return k + 1 + k%2
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}
	assert(reachNumber(2) == 3)
	assert(reachNumber(3) == 2)
}
